# rs_gitdiffcopy テスト結果

## 最新テスト実行結果

| 項目 | 内容 |
|-----|------|
| 実施日 | 2026-01-22 |
| 実施時刻 | 00:30 JST |
| 実行環境 | Linux (WSL2) |
| Rustバージョン | stable |
| 結果 | **ALL PASSED** |

## テスト結果サマリー

| カテゴリ | テスト数 | PASS | FAIL | スキップ |
|---------|---------|------|------|---------|
| ユニットテスト | 148 | 148 | 0 | 0 |
| 結合テスト | 151 | 151 | 0 | 0 |
| **合計** | **299** | **299** | **0** | **0** |

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

### 22. 不具合修正確認テスト (4テスト)

| テストID | テスト名 | 結果 | 備考 |
|---------|---------|------|------|
| BUGFIX-001 | test_summary_file_no_ansi_escape_codes | PASS | ANSIエスケープコード除去確認 |
| BUGFIX-002 | test_summary_file_status_alignment | PASS | ステータス位置整列確認 |
| BUGFIX-003 | test_excel_file_has_borders | PASS | Excel罫線確認 |
| BUGFIX-004 | test_summary_file_status_alignment_with_japanese | PASS | 日本語でのステータス整列確認 |

### 23. 三者間グループキーワードテスト (6テスト)

| テストID | テスト名 | 結果 | 備考 |
|---------|---------|------|------|
| GRP-001 | test_group_keyword_exclusion_added | PASS | ^addedグループ除外 |
| GRP-002 | test_group_keyword_exclusion_deleted | PASS | ^deletedグループ除外 |
| GRP-003 | test_group_keyword_exclusion_modified | PASS | ^modifiedグループ除外 |
| GRP-004 | test_group_keyword_exclusion_conflicts | PASS | ^conflictsグループ除外 |
| GRP-005 | test_group_keyword_inclusion_added | PASS | addedグループ包含 |
| GRP-006 | test_group_keyword_multiple | PASS | 複数グループ指定 |

### 24. 三者間File Tree整列テスト (3テスト)

| テストID | テスト名 | 結果 | 備考 |
|---------|---------|------|------|
| ALIGN-001 | test_three_way_summary_file_alignment | PASS | 三者間Summaryファイル整列 |
| ALIGN-002 | test_three_way_summary_file_alignment_japanese | PASS | 三者間日本語ファイル名整列 |
| ALIGN-003 | test_three_way_file_tree_alignment | PASS | 三者間File Tree整列（異なる深さ） |

### 25. ExcelファイルTree ディレクトリ分割形式テスト (4テスト)

| テストID | テスト名 | 結果 | 備考 |
|---------|---------|------|------|
| EXCEL-TREE-001 | test_excel_file_tree_directory_split_two_way | PASS | 二者間比較FileTreeディレクトリ分割 |
| EXCEL-TREE-002 | test_excel_file_tree_directory_split_three_way | PASS | 三者間比較FileTreeディレクトリ分割 |
| EXCEL-TREE-003 | test_excel_file_tree_status_column_position_two_way | PASS | 二者間比較Status列位置検証 |
| EXCEL-TREE-004 | test_excel_file_tree_status_column_position_three_way | PASS | 三者間比較Status列位置検証 |

### 26. Excelファイル グルーピング機能テスト (3テスト)

| テストID | テスト名 | 結果 | 備考 |
|---------|---------|------|------|
| EXCEL-GROUP-001 | test_excel_fold_level_two_way | PASS | 二者間比較--excel-fold-levelオプション |
| EXCEL-GROUP-002 | test_excel_fold_level_three_way | PASS | 三者間比較--excel-fold-levelオプション |
| EXCEL-GROUP-003 | test_excel_default_fold_level | PASS | デフォルトfold-level（グルーピングなし） |

---

## ユニットテスト詳細結果

### src/config.rs (17テスト)

| テスト名 | 結果 | 備考 |
|---------|------|------|
| test_input_config_parse_basic_toml | PASS | TOML基本パース |
| test_input_config_parse_with_exclude | PASS | excludeパターン |
| test_input_config_parse_with_filter_status | PASS | filter_status |
| test_input_config_parse_three_way_mode | PASS | 三者間モード設定 |
| test_merged_config_missing_source | PASS | 必須引数検証 |
| test_merged_config_missing_target | PASS | 必須引数検証 |
| test_merged_config_missing_output | PASS | 必須引数検証 |
| test_merged_config_three_way_requires_base | PASS | 三者間モード要件 |
| test_merged_config_three_way_with_base | PASS | 三者間モード正常 |
| test_merged_config_cli_priority_over_config | PASS | CLI優先順位 |
| test_merged_config_exclude_merge | PASS | excludeマージ |
| test_merged_config_filter_status_cli_priority | PASS | CLI優先 |
| test_merged_config_to_input_config_roundtrip | PASS | 変換往復 |
| test_merged_config_default_workers | PASS | デフォルト値 |
| test_merged_config_workers_priority | PASS | workers優先順位 |
| test_app_settings_default | PASS | デフォルト設定 |
| test_input_config_default | PASS | デフォルト設定 |

### src/git.rs (22テスト)

| テスト名 | 結果 | 備考 |
|---------|------|------|
| test_is_remote_url_https | PASS | HTTPSリモートURL判定 |
| test_is_remote_url_http | PASS | HTTPリモートURL判定 |
| test_is_remote_url_git_protocol | PASS | gitプロトコル判定 |
| test_is_remote_url_ssh | PASS | SSH URL判定 |
| test_is_remote_url_local_path | PASS | ローカルパス判定 |
| test_parse_diff_line_added | PASS | diff行パース（追加） |
| test_parse_diff_line_modified | PASS | diff行パース（変更） |
| test_parse_diff_line_deleted | PASS | diff行パース（削除） |
| test_parse_diff_line_renamed | PASS | diff行パース（リネーム） |
| test_parse_diff_line_renamed_partial_similarity | PASS | 類似度パース |
| test_parse_diff_line_copied | PASS | diff行パース（コピー） |
| test_parse_diff_line_type_changed | PASS | diff行パース（タイプ変更） |
| test_parse_diff_line_empty | PASS | 空行処理 |
| test_parse_diff_line_invalid_status | PASS | 不正ステータス |
| test_parse_diff_line_japanese_path | PASS | 日本語パス |
| test_parse_ls_tree_line_normal_file | PASS | ls-tree通常ファイル |
| test_parse_ls_tree_line_executable | PASS | 実行可能ファイル |
| test_parse_ls_tree_line_symlink | PASS | シンボリックリンク |
| test_parse_ls_tree_line_submodule | PASS | サブモジュール |
| test_parse_ls_tree_line_invalid | PASS | 不正行 |
| test_parse_ls_tree_line_japanese_path | PASS | 日本語パス |

### src/copy.rs (12テスト)

| テスト名 | 結果 | 備考 |
|---------|------|------|
| test_add_extension_simple | PASS | 拡張子追加 |
| test_add_extension_with_path | PASS | パス付き拡張子追加 |
| test_add_extension_deleted | PASS | .deleted拡張子 |
| test_add_extension_no_original_extension | PASS | 元拡張子なし |
| test_add_extension_dotfile | PASS | ドットファイル |
| test_add_extension_double_extension | PASS | 二重拡張子 |
| test_add_extension_japanese_filename | PASS | 日本語ファイル名 |
| test_add_extension_base | PASS | .base拡張子 |
| test_add_extension_ours | PASS | .ours拡張子 |
| test_add_extension_theirs | PASS | .theirs拡張子 |
| test_three_way_status_is_conflict_true | PASS | コンフリクト判定 |
| test_three_way_status_is_conflict_false | PASS | 非コンフリクト判定 |

### src/diff.rs (18テスト)

| テスト名 | 結果 | 備考 |
|---------|------|------|
| test_three_way_unchanged | PASS | 三者間：変更なし |
| test_three_way_ours_only | PASS | 三者間：oursのみ変更 |
| test_three_way_theirs_only | PASS | 三者間：theirsのみ変更 |
| test_three_way_both_same | PASS | 三者間：両方同じ変更 |
| test_three_way_conflict | PASS | 三者間：コンフリクト |
| test_three_way_deleted_ours | PASS | 三者間：ours削除 |
| test_three_way_deleted_theirs | PASS | 三者間：theirs削除 |
| test_three_way_deleted_both | PASS | 三者間：両方削除 |
| test_three_way_delete_modify_conflict | PASS | 三者間：削除/変更コンフリクト |
| test_three_way_modify_delete_conflict | PASS | 三者間：変更/削除コンフリクト |
| test_three_way_added_ours | PASS | 三者間：ours追加 |
| test_three_way_added_theirs | PASS | 三者間：theirs追加 |
| test_three_way_added_both_same | PASS | 三者間：両方同じ内容追加 |
| test_three_way_added_both_diff | PASS | 三者間：両方異なる内容追加 |
| test_three_way_none_everywhere | PASS | 三者間：存在しない |
| test_calculate_statistics_empty | PASS | 空統計計算 |
| test_diff_file_is_permission_only_change | PASS | 権限のみ変更判定 |
| test_diff_file_is_not_permission_only_change | PASS | 内容変更判定 |

### src/summary.rs (30テスト)

| テスト名 | 結果 | 備考 |
|---------|------|------|
| test_format_status_tag_added_no_color | PASS | ステータスタグ（カラーなし） |
| test_format_status_tag_modified_no_color | PASS | ステータスタグ（カラーなし） |
| test_format_status_tag_deleted_no_color | PASS | ステータスタグ（カラーなし） |
| test_format_status_tag_renamed_no_color | PASS | ステータスタグ（カラーなし） |
| test_format_status_tag_copied_no_color | PASS | ステータスタグ（カラーなし） |
| test_format_status_tag_type_changed_no_color | PASS | ステータスタグ（カラーなし） |
| test_format_status_tag_unchanged_no_color | PASS | ステータスタグ（カラーなし） |
| test_format_status_tag_added_with_color | PASS | ステータスタグ（カラーあり） |
| test_build_tree_single_file | PASS | ツリー構築（単一ファイル） |
| test_build_tree_nested_file | PASS | ツリー構築（ネスト） |
| test_build_tree_multiple_files_same_dir | PASS | ツリー構築（複数ファイル） |
| test_build_tree_deep_nesting | PASS | ツリー構築（深いネスト） |
| test_build_tree_japanese_path | PASS | ツリー構築（日本語パス） |
| test_calculate_max_tree_width_single_file | PASS | ツリー幅計算（単一） |
| test_calculate_max_tree_width_long_filename | PASS | ツリー幅計算（長いファイル名） |
| test_calculate_max_tree_width_nested_long | PASS | ツリー幅計算（ネスト） |
| test_calculate_max_tree_width_japanese | PASS | ツリー幅計算（日本語） |
| test_statistics_total | PASS | 統計合計 |
| test_three_way_statistics_total | PASS | 三者間統計合計 |
| test_three_way_statistics_conflicts | PASS | 三者間コンフリクト数 |
| test_display_width_ascii | PASS | ASCII文字幅計算 |
| test_display_width_japanese | PASS | 日本語文字幅計算 |
| test_display_width_box_drawing | PASS | Box Drawing文字幅計算（幅2） |
| test_display_width_emoji | PASS | 絵文字幅計算 |
| test_display_width_mixed | PASS | 混合文字幅計算 |
| test_display_width_box_drawing_tree | PASS | ツリー罫線幅計算 |

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

### src/excel.rs (8テスト)

| テスト名 | 結果 | 備考 |
|---------|------|------|
| test_calculate_row_groups_empty | PASS | 空配列グルーピング |
| test_calculate_row_groups_all_below_fold_level | PASS | 全て基準以下 |
| test_calculate_row_groups_single_group | PASS | 単一グループ |
| test_calculate_row_groups_multiple_groups | PASS | 複数グループ |
| test_calculate_row_groups_fold_level_1 | PASS | 折りたたみレベル1 |
| test_calculate_row_groups_at_end | PASS | 末尾グループ |
| test_calculate_row_groups_alternating | PASS | 交互深度 |
| test_calculate_row_groups_all_same_depth | PASS | 全て同深度 |

### src/types.rs (37テスト)

| テスト名 | 結果 | 備考 |
|---------|------|------|
| test_file_status_from_git_status | PASS | |
| test_file_status_tag | PASS | |
| test_file_status_from_filter_str | PASS | |
| test_three_way_status_is_conflict | PASS | |
| test_three_way_status_tag | PASS | |
| test_three_way_status_from_filter_str | PASS | フィルター文字列からステータス解析 |
| test_three_way_status_from_filter_str_case_insensitive | PASS | 大文字小文字区別なし |
| test_three_way_status_from_filter_str_underscore | PASS | アンダースコア形式対応 |
| test_three_way_status_from_filter_str_invalid | PASS | 無効値処理 |
| test_three_way_status_all | PASS | 全ステータスリスト取得 |
| test_filter_group_from_str | PASS | グループキーワード解析 |
| test_filter_group_expand_added | PASS | addedグループ展開 |
| test_filter_group_expand_modified | PASS | modifiedグループ展開 |
| test_filter_group_expand_deleted | PASS | deletedグループ展開 |
| test_filter_group_expand_conflicts | PASS | conflictsグループ展開 |
| test_filter_group_expand_all | PASS | allグループ展開 |
| test_expand_filter_status_three_way_empty | PASS | 空フィルター処理 |
| test_expand_filter_status_three_way_single_status | PASS | 単一ステータス |
| test_expand_filter_status_three_way_group_added | PASS | addedグループ指定 |
| test_expand_filter_status_three_way_group_conflicts | PASS | conflictsグループ指定 |
| test_expand_filter_status_three_way_exclude | PASS | 除外指定 |
| test_expand_filter_status_three_way_all_exclude | PASS | all,^除外 |
| test_expand_filter_status_three_way_multiple_groups | PASS | 複数グループ指定 |
| test_expand_filter_status_three_way_exclude_conflicts | PASS | ^conflicts除外 |
| test_expand_filter_status_three_way_mixed_include_exclude | PASS | 包含・除外混合 |
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
| 2026-01-21 | 22:40 | 26/26 | 135/135 | ALL PASSED | 不具合修正テスト追加（4件） |
| 2026-01-21 | 23:30 | 114/114 | 135/135 | ALL PASSED | Unitテスト大幅追加（88件追加） |
| 2026-01-21 | 23:50 | 134/134 | 144/144 | ALL PASSED | 三者間グループキーワード機能追加、Unit+結合テスト追加（29件追加） |
| 2026-01-22 | 00:30 | 148/148 | 149/149 | ALL PASSED | Excel行グルーピング機能、FileTree分割テスト追加（19件追加） |
| 2026-01-22 | 01:00 | 148/148 | 151/151 | ALL PASSED | Status列位置検証テスト追加（2件追加）、status_col修正対応 |

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
| 2026-01-21 | types.rs に三者間グループキーワード機能を追加（FilterGroup, expand_filter_status_three_way） |
| 2026-01-21 | main.rs で三者間比較時のfilter_status表示フィルタリングを追加 |
| 2026-01-21 | copy.rs で三者間比較時のfilter_statusコピーフィルタリングを追加 |
| 2026-01-22 | excel.rs にExcel行グルーピング機能を追加（apply_row_grouping, calculate_row_groups） |
| 2026-01-22 | summary.rs のdisplay_width関数でBox Drawing文字を幅2として計算するよう修正 |
| 2026-01-22 | Cargo.toml の rust_xlsxwriter を0.92に更新（group_rows API対応） |
| 2026-01-22 | excel.rs のwrite_file_tree_sheet, write_three_way_file_tree_sheetでstatus_col計算を修正（max_depth + 1）|
