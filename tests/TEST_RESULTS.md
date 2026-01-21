# rs_gitdiffcopy テスト結果

## 最新テスト実行結果

| 項目 | 内容 |
|-----|------|
| 実施日 | 2026-01-21 |
| 実施時刻 | 20:50 JST |
| 実行環境 | Linux (WSL2) |
| Rustバージョン | stable |
| 結果 | **ALL PASSED** |

## テスト結果サマリー

| カテゴリ | テスト数 | PASS | FAIL | スキップ |
|---------|---------|------|------|---------|
| ユニットテスト | 26 | 26 | 0 | 0 |
| 結合テスト | 131 | 131 | 0 | 0 |
| **合計** | **157** | **157** | **0** | **0** |

---

## 結合テスト詳細結果

### 1. 基本動作テスト (4テスト)

| テストID | テスト名 | 結果 | 備考 |
|---------|---------|------|------|
| BASIC-001 | test_help_option | PASS | |
| BASIC-002 | test_help_short_option | PASS | |
| BASIC-003 | test_version_option | PASS | |
| BASIC-004 | test_version_short_option | PASS | |

### 2. 二者間比較テスト (12テスト)

| テストID | テスト名 | 結果 | 備考 |
|---------|---------|------|------|
| TWO-001 | test_added_file_detection | PASS | |
| TWO-002 | test_modified_file_detection | PASS | |
| TWO-003 | test_deleted_file_detection | PASS | |
| TWO-004 | test_renamed_file_detection | PASS | |
| TWO-005 | test_unchanged_file_detection | PASS | |
| TWO-006 | test_directory_structure_preserved | PASS | |
| TWO-007 | test_no_differences_exit_code | PASS | |
| TWO-008 | test_differences_exit_code | PASS | |
| TWO-009 | test_branch_comparison | PASS | |
| TWO-010 | test_tag_comparison | PASS | |
| TWO-011 | test_commit_hash_comparison | PASS | |
| TWO-012 | test_head_relative_comparison | PASS | |

### 3. 三者間比較テスト (4テスト)

| テストID | テスト名 | 結果 | 備考 |
|---------|---------|------|------|
| THREE-001 | test_three_way_basic | PASS | |
| THREE-002 | test_three_way_conflict | PASS | |
| THREE-003 | test_three_way_both_same_change | PASS | |
| THREE-004 | test_three_way_requires_base | PASS | |

### 4. オプションテスト (17テスト)

| テストID | テスト名 | 結果 | 備考 |
|---------|---------|------|------|
| OPT-001 | test_dry_run | PASS | |
| OPT-002 | test_both_versions | PASS | |
| OPT-003 | test_copy_deleted | PASS | |
| OPT-005 | test_exclude_pattern | PASS | |
| OPT-006 | test_multiple_exclude_patterns | PASS | |
| OPT-010 | test_summary_file | PASS | |
| OPT-011 | test_verbose_mode | PASS | |
| OPT-012 | test_stats_only | PASS | |
| OPT-013 | test_filter_status_added | PASS | |
| OPT-014 | test_filter_status_modified | PASS | |
| OPT-016 | test_no_tree_option | PASS | |
| OPT-017 | test_no_details_option | PASS | |
| OPT-018 | test_workers_option | PASS | |

### 5. 日本語パステスト (12テスト)

| テストID | テスト名 | 結果 | 備考 |
|---------|---------|------|------|
| JP-001 | test_japanese_directory_names | PASS | |
| JP-002 | test_japanese_file_names | PASS | |
| JP-003 | test_japanese_nested_path | PASS | |
| JP-004 | test_mixed_japanese_english_path | PASS | |
| JP-005 | test_japanese_in_summary_output | PASS | |
| JP-006 | test_japanese_both_versions | PASS | |
| JP-007 | test_japanese_copy_deleted | PASS | |
| JP-012 | test_japanese_content_in_file | PASS | |
| JP-013 | test_hiragana_katakana_kanji_mixed | PASS | |
| JP-014 | test_long_japanese_filename | PASS | |

### 6. エラーハンドリングテスト (6テスト)

| テストID | テスト名 | 結果 | 備考 |
|---------|---------|------|------|
| ERR-001 | test_nonexistent_repository | PASS | |
| ERR-002 | test_nonexistent_source_ref | PASS | |
| ERR-003 | test_nonexistent_target_ref | PASS | |
| ERR-004 | test_output_exists_without_force | PASS | |
| ERR-006 | test_missing_required_args | PASS | |
| ERR-007 | test_not_git_repository | PASS | |

### 7. 終了コードテスト (4テスト)

| テストID | テスト名 | 結果 | 備考 |
|---------|---------|------|------|
| EXIT-001 | test_exit_code_0_with_differences | PASS | |
| EXIT-002 | test_exit_code_1_on_error | PASS | |
| EXIT-003 | test_exit_code_2_no_differences | PASS | |
| EXIT-004 | test_exit_code_3_conflicts | PASS | |

### 8. 設定ファイルテスト (4テスト)

| テストID | テスト名 | 結果 | 備考 |
|---------|---------|------|------|
| CFG-001 | test_config_file_basic | PASS | |
| CFG-002 | test_config_file_with_exclude | PASS | |
| CFG-003 | test_cli_overrides_config | PASS | |
| CFG-004 | test_save_config | PASS | |

### 9. show_unchangedテスト (3テスト)

| テストID | テスト名 | 結果 | 備考 |
|---------|---------|------|------|
| SHOW-001 | test_show_unchanged_option | PASS | |
| SHOW-002 | test_show_unchanged_short_option | PASS | |
| SHOW-003 | test_unchanged_count_in_statistics | PASS | |

### 10. Git ref形式テスト (5テスト)

| テストID | テスト名 | 結果 | 備考 |
|---------|---------|------|------|
| REF-001 | test_branch_name_with_slash | PASS | |
| REF-002 | test_short_commit_hash | PASS | |
| REF-003 | test_full_commit_hash | PASS | |
| REF-004 | test_head_caret_notation | PASS | |
| REF-005 | test_head_tilde_notation | PASS | |

### 11. サマリーファイル内容検証テスト (7テスト)

| テストID | テスト名 | 結果 | 備考 |
|---------|---------|------|------|
| SUM-001 | test_summary_contains_header | PASS | |
| SUM-002 | test_summary_statistics_accuracy | PASS | |
| SUM-003 | test_summary_contains_file_tree | PASS | |
| SUM-005 | test_summary_japanese_paths | PASS | |
| SUM-006 | test_summary_options_section | PASS | |
| SUM-007 | test_summary_no_differences | PASS | |

### 12. パッチファイルテスト (10テスト)

| テストID | テスト名 | 結果 | 備考 |
|---------|---------|------|------|
| PATCH-001 | test_patch_file_unified_format | PASS | |
| PATCH-002 | test_combined_patch_file | PASS | |
| PATCH-003 | test_patch_japanese_content | PASS | |
| PATCH-004 | test_patch_binary_skipped | PASS | |
| PATCH-005 | test_patch_addition_only | PASS | |
| PATCH-006 | test_patch_deletion_only | PASS | |
| PATCH-007 | test_patch_multiple_hunks | PASS | |
| PATCH-008 | test_patch_hunk_header_format | PASS | |
| PATCH-009 | test_patch_subdirectory | PASS | |
| PATCH-010 | test_patch_and_patch_file_together | PASS | |

### 13. Excelファイル内容検証テスト (4テスト)

| テストID | テスト名 | 結果 | 備考 |
|---------|---------|------|------|
| EXCEL-001 | test_excel_has_correct_sheets | PASS | |
| EXCEL-002 | test_excel_summary_statistics | PASS | |
| EXCEL-003 | test_excel_file_tree_entries | PASS | |
| EXCEL-004 | test_excel_japanese_filenames | PASS | |

### 14. エッジケーステスト (5テスト)

| テストID | テスト名 | 結果 | 備考 |
|---------|---------|------|------|
| EDGE-001 | test_empty_file | PASS | |
| EDGE-002 | test_same_size_different_content | PASS | |
| EDGE-003 | test_deeply_nested_directories | PASS | |
| EDGE-004 | test_unicode_russian_filenames | PASS | |
| EDGE-005 | test_exclude_pycache_pattern | PASS | |

### 15. シンボリックリンクテスト (3テスト)

| テストID | テスト名 | 結果 | 備考 |
|---------|---------|------|------|
| SYM-001 | test_symlink_added_detection | PASS | Unix only |
| SYM-002 | test_symlink_deleted_detection | PASS | Unix only |
| SYM-003 | test_broken_symlink_detection | PASS | Unix only |

### 16. パーミッションテスト (2テスト)

| テストID | テスト名 | 結果 | 備考 |
|---------|---------|------|------|
| PERM-001 | test_permission_check_default_disabled | PASS | Unix only |
| PERM-002 | test_permission_check_scripts_mode | PASS | Unix only |

### 17. 拡張三者間比較テスト (9テスト)

| テストID | テスト名 | 結果 | 備考 |
|---------|---------|------|------|
| EXT3-001 | test_three_way_added_ours | PASS | |
| EXT3-002 | test_three_way_added_theirs | PASS | |
| EXT3-003 | test_three_way_deleted_ours | PASS | |
| EXT3-004 | test_three_way_deleted_theirs | PASS | |
| EXT3-005 | test_three_way_deleted_both | PASS | |
| EXT3-006 | test_three_way_modify_delete_conflict | PASS | |
| EXT3-007 | test_three_way_merge_style_ours | PASS | |
| EXT3-008 | test_three_way_merge_style_theirs | PASS | |
| EXT3-009 | test_three_way_conflict_only | PASS | |

### 18. Excelフォーマット詳細テスト (10テスト)

| テストID | テスト名 | 結果 | 備考 |
|---------|---------|------|------|
| EXFMT-001 | test_excel_file_generation | PASS | |
| EXFMT-002 | test_excel_fold_level_option | PASS | |
| EXFMT-003 | test_excel_fold_level_zero_default | PASS | |
| EXFMT-004 | test_excel_subdirectory_structure | PASS | |
| EXFMT-005 | test_excel_summary_has_labels | PASS | |
| EXFMT-006 | test_excel_summary_has_statistics_section | PASS | |
| EXFMT-007 | test_excel_summary_has_options_section | PASS | |
| EXFMT-008 | test_excel_details_has_header | PASS | |
| EXFMT-009 | test_excel_details_sections | PASS | |
| EXFMT-010 | test_excel_file_tree_cell_structure | PASS | |

### 19. Filter statusテスト (7テスト)

| テストID | テスト名 | 結果 | 備考 |
|---------|---------|------|------|
| FILT-001 | test_filter_status_added | PASS | |
| FILT-002 | test_filter_status_modified | PASS | |
| FILT-003 | test_filter_status_multiple_values | PASS | |
| FILT-004 | test_no_filter_status_when_not_specified | PASS | |
| FILT-005 | test_excel_contains_filter_status_option | PASS | |
| FILT-006 | test_excel_no_filter_status_when_not_specified | PASS | |
| FILT-007 | test_summary_contains_filter_status_option | PASS | |

### 20. 統計テスト (6テスト)

| テストID | テスト名 | 結果 | 備考 |
|---------|---------|------|------|
| STATS-001 | test_unchanged_count_always_shown | PASS | |
| STATS-002 | test_unchanged_count_with_many_files | PASS | |
| STATS-003 | test_unchanged_count_same_with_or_without_option | PASS | |
| STATS-004 | test_statistics_categories_sum | PASS | |
| STATS-005 | test_statistics_with_filter | PASS | |
| STATS-006 | test_total_equals_all_unique_paths | PASS | |

### 21. Tree表示テスト (5テスト)

| テストID | テスト名 | 結果 | 備考 |
|---------|---------|------|------|
| TREE-001 | test_console_output_contains_box_drawing_chars | PASS | |
| TREE-002 | test_summary_file_contains_box_drawing_chars | PASS | |
| TREE-003 | test_box_drawing_chars_at_different_depths | PASS | |
| TREE-004 | test_tree_structure_formatting | PASS | |
| TREE-005 | test_three_way_tree_contains_box_drawing_chars | PASS | |

---

## ユニットテスト詳細結果

### src/safety.rs (12テスト)

| テスト名 | 結果 | 備考 |
|---------|------|------|
| test_is_protected_path_unix_root | PASS | PROT-001 |
| test_is_protected_path_unix_system_dirs | PASS | PROT-002 |
| test_is_protected_path_unix_safe_paths | PASS | PROT-003 |
| test_validate_output_path_protected | PASS | PROT-004 |
| test_validate_output_path_safe | PASS | PROT-005 |
| test_is_protected_path_home_directory | PASS | |
| test_format_size | PASS | |
| test_get_directory_info_empty | PASS | |
| test_get_directory_info_with_files | PASS | |
| test_remove_directory | PASS | |
| test_remove_nonexistent_directory | PASS | |

### src/types.rs (14テスト)

| テスト名 | 結果 | 備考 |
|---------|------|------|
| test_file_status_from_git_status | PASS | |
| test_file_status_tag | PASS | |
| test_file_status_from_filter_str | PASS | |
| test_three_way_status_is_conflict | PASS | |
| test_three_way_status_tag | PASS | |
| test_diff_file_new | PASS | |
| test_diff_file_is_permission_only_change | PASS | |
| test_permission_check_mode_from_str | PASS | |
| test_merge_style_from_str | PASS | |
| test_color_mode_from_str | PASS | |
| test_log_level_from_str | PASS | |
| test_log_level_to_filter | PASS | |
| test_statistics_total | PASS | |
| test_three_way_statistics_total | PASS | |
| test_three_way_statistics_conflicts | PASS | |

---

## テスト実行履歴

| 日付 | 時刻 | Unit | Integration | 結果 | 備考 |
|------|------|------|-------------|------|------|
| 2026-01-21 | 20:15 | 26/26 | 71/71 | ALL PASSED | 初回テスト実施 |
| 2026-01-21 | 20:50 | 26/26 | 131/131 | ALL PASSED | テスト追加（60件追加） |

---

## テスト実行方法

```bash
# すべてのテストを実行
cargo test

# ユニットテストのみ
cargo test --lib

# 結合テストのみ
cargo test --test integration_tests

# 特定のテストを実行
cargo test --test integration_tests test_japanese

# 詳細出力付き
cargo test -- --nocapture

# テスト結果をファイルに保存
cargo test 2>&1 | tee test_output.txt
```

---

## 注意事項

- 保護ディレクトリテスト（PROT-001〜005）はユニットテスト（src/safety.rs）でのみ実行
- システムディレクトリへの誤操作を防ぐため、結合テストでは保護ディレクトリの実際の削除テストは行わない
- 日本語パステストは UTF-8 対応環境で実行すること
- テストにはGitコマンドが必要（PATH上にgitが存在すること）
- WSL2環境では /bin 等がシンボリックリンクのため、一部のテストは条件付きで実行
- シンボリックリンクテスト、パーミッションテストはUnix環境でのみ有効

---

## 修正履歴

| 日付 | 内容 |
|------|------|
| 2026-01-21 | diff.rs の fetch_file_info 関数を修正（差分ファイル処理が正しく行われるよう修正） |
| 2026-01-21 | copy.rs の should_copy 関数を修正（filter_status オプションのサポートを追加） |
| 2026-01-21 | テストリポジトリで core.quotepath=false を設定（日本語ファイル名対応） |
| 2026-01-21 | main.rs に統合パッチファイル生成機能を追加（-F/--patch-file オプション） |
