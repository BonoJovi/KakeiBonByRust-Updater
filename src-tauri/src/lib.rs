use sqlx::sqlite::SqlitePool;
use std::path::PathBuf;

const DB_DIR_NAME: &str = ".kakeibon";
const DB_FILE_NAME: &str = "KakeiBonDB.sqlite3";

/// SQL statements to add dashboard i18n resources
const UPDATE_SQLS: &[&str] = &[
    // Admin access denied messages for transaction management
    "INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1127, 'transaction.admin_access_denied', 'en', 'Transaction management is not available for administrator accounts. Please login as a regular user.', 'transaction_mgmt', 'Admin access denied message', datetime('now'))",
    "INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (1128, 'transaction.admin_access_denied', 'ja', '入出金管理は管理者アカウントでは利用できません。一般ユーザーでログインしてください。', 'transaction_mgmt', '管理者アクセス拒否メッセージ', datetime('now'))",

    // Dashboard menu item
    "INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (2001, 'menu.dashboard', 'en', 'Dashboard', 'menu', 'Dashboard menu item', datetime('now'))",
    "INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (2002, 'menu.dashboard', 'ja', 'ダッシュボード', 'menu', 'ダッシュボードメニュー項目', datetime('now'))",

    // Dashboard page
    "INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (2003, 'dashboard.title', 'en', 'Dashboard', 'dashboard', 'Dashboard page title', datetime('now'))",
    "INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (2004, 'dashboard.title', 'ja', 'ダッシュボード', 'dashboard', 'ダッシュボードページタイトル', datetime('now'))",
    "INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (2005, 'dashboard.filter', 'en', 'Filter', 'dashboard', 'Filter section header', datetime('now'))",
    "INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (2006, 'dashboard.filter', 'ja', 'フィルター', 'dashboard', 'フィルターセクションヘッダー', datetime('now'))",
    "INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (2007, 'dashboard.year', 'en', 'Year:', 'dashboard', 'Year label', datetime('now'))",
    "INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (2008, 'dashboard.year', 'ja', '年:', 'dashboard', '年ラベル', datetime('now'))",
    "INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (2009, 'dashboard.month', 'en', 'Month:', 'dashboard', 'Month label', datetime('now'))",
    "INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (2010, 'dashboard.month', 'ja', '月:', 'dashboard', '月ラベル', datetime('now'))",
    "INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (2011, 'dashboard.trend_months', 'en', 'Aggregation Period:', 'dashboard', 'Aggregation period label', datetime('now'))",
    "INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (2012, 'dashboard.trend_months', 'ja', '集計期間:', 'dashboard', '集計期間ラベル', datetime('now'))",
    "INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (2013, 'dashboard.execute', 'en', 'Execute', 'dashboard', 'Execute button', datetime('now'))",
    "INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (2014, 'dashboard.execute', 'ja', '集計', 'dashboard', '集計ボタン', datetime('now'))",

    // Chart titles
    "INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (2015, 'dashboard.expense_by_category', 'en', 'Expense by Category', 'dashboard', 'Pie chart title', datetime('now'))",
    "INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (2016, 'dashboard.expense_by_category', 'ja', 'カテゴリ別支出', 'dashboard', '円グラフタイトル', datetime('now'))",
    "INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (2017, 'dashboard.category_comparison', 'en', 'Category Comparison', 'dashboard', 'Bar chart title', datetime('now'))",
    "INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (2018, 'dashboard.category_comparison', 'ja', 'カテゴリ別比較', 'dashboard', '棒グラフタイトル', datetime('now'))",
    "INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (2019, 'dashboard.monthly_trend', 'en', 'Monthly Trend', 'dashboard', 'Line chart title', datetime('now'))",
    "INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (2020, 'dashboard.monthly_trend', 'ja', '月別推移', 'dashboard', '折れ線グラフタイトル', datetime('now'))",

    // Chart labels
    "INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (2021, 'dashboard.expense', 'en', 'Expense', 'dashboard', 'Expense label', datetime('now'))",
    "INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (2022, 'dashboard.expense', 'ja', '支出', 'dashboard', '支出ラベル', datetime('now'))",
    "INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (2023, 'dashboard.income', 'en', 'Income', 'dashboard', 'Income label', datetime('now'))",
    "INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (2024, 'dashboard.income', 'ja', '収入', 'dashboard', '収入ラベル', datetime('now'))",
    "INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (2025, 'dashboard.balance', 'en', 'Balance', 'dashboard', 'Balance label', datetime('now'))",
    "INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (2026, 'dashboard.balance', 'ja', '収支', 'dashboard', '収支ラベル', datetime('now'))",

    // Messages
    "INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (2027, 'dashboard.no_data', 'en', 'No data available', 'dashboard', 'No data message', datetime('now'))",
    "INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (2028, 'dashboard.no_data', 'ja', 'データがありません', 'dashboard', 'データなしメッセージ', datetime('now'))",
    "INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (2029, 'dashboard.loading', 'en', 'Loading...', 'dashboard', 'Loading message', datetime('now'))",
    "INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (2030, 'dashboard.loading', 'ja', '読み込み中...', 'dashboard', '読み込み中メッセージ', datetime('now'))",

    // Errors
    "INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (2031, 'dashboard.error_invalid_year', 'en', 'Invalid year', 'dashboard', 'Invalid year error', datetime('now'))",
    "INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (2032, 'dashboard.error_invalid_year', 'ja', '無効な年です', 'dashboard', '無効な年エラー', datetime('now'))",
    "INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (2033, 'dashboard.error_invalid_month', 'en', 'Invalid month', 'dashboard', 'Invalid month error', datetime('now'))",
    "INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (2034, 'dashboard.error_invalid_month', 'ja', '無効な月です', 'dashboard', '無効な月エラー', datetime('now'))",

    // Access control
    "INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (2035, 'dashboard.admin_access_denied', 'en', 'Dashboard is not available for administrator accounts. Please login as a regular user.', 'dashboard', 'Admin access denied message', datetime('now'))",
    "INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (2036, 'dashboard.admin_access_denied', 'ja', 'ダッシュボードは管理者アカウントでは利用できません。一般ユーザーでログインしてください。', 'dashboard', '管理者アクセス拒否メッセージ', datetime('now'))",

    // Period display suffixes
    "INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (2037, 'dashboard.year_suffix', 'en', '/', 'dashboard', 'Year suffix for period display', datetime('now'))",
    "INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (2038, 'dashboard.year_suffix', 'ja', '年', 'dashboard', '期間表示の年サフィックス', datetime('now'))",
    "INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (2039, 'dashboard.month_suffix', 'en', '', 'dashboard', 'Month suffix for period display', datetime('now'))",
    "INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT) VALUES (2040, 'dashboard.month_suffix', 'ja', '月', 'dashboard', '期間表示の月サフィックス', datetime('now'))",
];

#[derive(serde::Serialize)]
struct UpdateResult {
    success: bool,
    message: String,
    inserted_count: usize,
    skipped_count: usize,
}

fn get_db_path() -> Option<PathBuf> {
    dirs::home_dir().map(|home| home.join(DB_DIR_NAME).join(DB_FILE_NAME))
}

#[tauri::command]
async fn check_database() -> Result<String, String> {
    let db_path = get_db_path().ok_or("Could not determine home directory")?;

    if db_path.exists() {
        Ok(format!("Database found: {}", db_path.display()))
    } else {
        Err(format!("Database not found: {}", db_path.display()))
    }
}

#[tauri::command]
async fn run_update() -> Result<UpdateResult, String> {
    let db_path = get_db_path().ok_or("Could not determine home directory")?;

    if !db_path.exists() {
        return Err(format!(
            "KakeiBon database not found at: {}\nPlease run KakeiBon at least once before using this updater.",
            db_path.display()
        ));
    }

    let db_url = format!("sqlite://{}", db_path.display());
    let pool = SqlitePool::connect(&db_url)
        .await
        .map_err(|e| format!("Failed to connect to database: {}", e))?;

    let mut inserted_count = 0;
    let mut skipped_count = 0;

    for sql in UPDATE_SQLS {
        let result = sqlx::query(sql)
            .execute(&pool)
            .await
            .map_err(|e| format!("SQL execution failed: {}", e))?;

        if result.rows_affected() > 0 {
            inserted_count += 1;
        } else {
            skipped_count += 1;
        }
    }

    pool.close().await;

    let message = if inserted_count > 0 {
        format!(
            "Update completed!\nInserted: {} records\nSkipped (already exists): {} records",
            inserted_count, skipped_count
        )
    } else {
        "Database is already up to date. No new records were added.".to_string()
    };

    Ok(UpdateResult {
        success: true,
        message,
        inserted_count,
        skipped_count,
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![check_database, run_update])
        .setup(|_app| {
            // Open devtools in debug builds
            #[cfg(debug_assertions)]
            {
                use tauri::Manager;
                if let Some(window) = _app.get_webview_window("main") {
                    window.open_devtools();
                }
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
