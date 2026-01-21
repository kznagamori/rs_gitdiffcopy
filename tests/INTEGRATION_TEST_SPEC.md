# rs_gitdiffcopy 結合テスト仕様書

## 概要

このドキュメントは、rs_gitdiffcopyの結合テスト仕様を定義します。
結合テストは、コマンドライン引数を使用した実際の動作をテストします。

## テスト結果の記録

テスト実行後は、以下のファイルに結果を記録してください：

```
tests/TEST_RESULTS.md
```

### 結果ファイルの更新方法

```bash
# テスト実行
cargo test --test integration_tests -- --test-threads=1 2>&1 | tee test_output.txt

# 結果をTEST_RESULTS.mdに記録
# - 実施日
# - 実行環境
# - テスト結果（PASS/FAIL）
# - 失敗時のエラー詳細
```

## テスト環境

- テスト用の一時ディレクトリを使用
- 各テストは独立して実行可能
- 各テストでGitリポジトリを動的に作成
- 日本語パスのテストを含む

## テストカテゴリと詳細

---

### 1. 基本動作テスト

| テストID | テスト名 | テスト内容 | 期待結果 | 分類 |
|---------|---------|-----------|---------|------|
| BASIC-001 | test_help_option | `--help`オプション実行 | ヘルプが表示され、終了コード0 | 正常系 |
| BASIC-002 | test_help_short_option | `-h`オプション実行 | ヘルプが表示され、終了コード0 | 正常系 |
| BASIC-003 | test_version_option | `--version`オプション実行 | バージョン表示、終了コード0 | 正常系 |
| BASIC-004 | test_version_short_option | `-V`オプション実行 | バージョン表示、終了コード0 | 正常系 |

---

### 2. 二者間比較テスト

| テストID | テスト名 | テスト内容 | 期待結果 | 分類 |
|---------|---------|-----------|---------|------|
| TWO-001 | test_added_file_detection | targetでファイルを追加 | 「Added」として検出、出力にコピー | 正常系 |
| TWO-002 | test_modified_file_detection | sourceとtargetで内容が異なる | 「Modified」として検出、新内容をコピー | 正常系 |
| TWO-003 | test_deleted_file_detection | sourceのみにファイルが存在 | 「Deleted」として検出、コピーされない | 正常系 |
| TWO-004 | test_renamed_file_detection | ファイル名が変更 | 「Renamed」として検出 | 正常系 |
| TWO-005 | test_unchanged_file_detection | sourceとtargetで内容が同一 | 「Unchanged」として検出、コピーされない | 正常系 |
| TWO-006 | test_directory_structure_preserved | ネストしたディレクトリ構造 | ディレクトリ構造が出力に保持 | 正常系 |
| TWO-007 | test_no_differences_exit_code | 差分なし | 終了コード2 | 正常系 |
| TWO-008 | test_differences_exit_code | 差分あり | 終了コード0 | 正常系 |
| TWO-009 | test_branch_comparison | ブランチ間の比較 | 正常に比較・コピー | 正常系 |
| TWO-010 | test_tag_comparison | タグ間の比較 | 正常に比較・コピー | 正常系 |
| TWO-011 | test_commit_hash_comparison | コミットハッシュ指定 | 正常に比較・コピー | 正常系 |
| TWO-012 | test_head_relative_comparison | HEAD相対参照（HEAD~1） | 正常に比較・コピー | 正常系 |

---

### 3. 三者間比較テスト

| テストID | テスト名 | テスト内容 | 期待結果 | 分類 |
|---------|---------|-----------|---------|------|
| THREE-001 | test_three_way_basic | 基本的な三者間比較 | 正常に動作 | 正常系 |
| THREE-002 | test_three_way_conflict | 両方で異なる変更 | コンフリクト検出、終了コード3 | 正常系 |
| THREE-003 | test_three_way_both_same_change | 両方で同一の変更 | 「both-same」として検出 | 正常系 |
| THREE-004 | test_three_way_requires_base | --baseなしで--three-way | エラー終了 | 準正常系 |
| THREE-005 | test_three_way_ours_only_change | Oursのみ変更 | ours-onlyとして検出 | 正常系 |
| THREE-006 | test_three_way_theirs_only_change | Theirsのみ変更 | theirs-onlyとして検出 | 正常系 |
| THREE-007 | test_three_way_file_added_both | 両方で新規追加（異なる内容） | コンフリクト検出 | 準正常系 |
| THREE-008 | test_three_way_file_deleted_both | 両方で削除 | 正常検出 | 準正常系 |

---

### 3.1 三者間比較ファイル出力形式テスト

| テストID | テスト名 | テスト内容 | 期待結果 | 分類 |
|---------|---------|-----------|---------|------|
| THREE-COPY-001 | test_three_way_copy_output_format_status_suffix | ファイル出力形式のステータスサフィックス | `file.txt.added-ours`, `file.txt.added-theirs`形式で出力 | 正常系 |
| THREE-COPY-002 | test_three_way_copy_output_ours_theirs_only | OursOnly/TheirsOnlyステータスの出力形式 | `file.txt.ours-only`形式で出力 | 正常系 |
| THREE-COPY-003 | test_three_way_copy_output_both_same | BothSameステータスの出力形式 | `file.txt.both-same`形式で出力 | 正常系 |
| THREE-COPY-004 | test_three_way_copy_output_conflict | Conflictステータスの出力形式 | `file.txt.conflict.base`, `.conflict.ours`, `.conflict.theirs`形式で出力 | 正常系 |
| THREE-COPY-005 | test_three_way_copy_output_nested_directory | ネストされたディレクトリでの出力形式 | ディレクトリ構造を保持しステータスサフィックス付きで出力 | 正常系 |

---

### 4. オプションテスト

| テストID | テスト名 | テスト内容 | 期待結果 | 分類 |
|---------|---------|-----------|---------|------|
| OPT-001 | test_dry_run | --dry-runオプション | ファイルがコピーされない | 正常系 |
| OPT-002 | test_both_versions | --both-versionsオプション | .oldと.newが作成 | 正常系 |
| OPT-003 | test_copy_deleted | --copy-deletedオプション | .deletedファイルが作成 | 正常系 |
| OPT-004 | test_preserve_timestamps | --preserve-timestampsオプション | タイムスタンプが保持 | 正常系 |
| OPT-005 | test_exclude_pattern | --excludeオプション（単一） | パターンに一致するファイルが除外 | 正常系 |
| OPT-006 | test_multiple_exclude_patterns | --excludeオプション（複数） | 複数パターンが除外 | 正常系 |
| OPT-007 | test_patch_generation | --patchオプション | .patchファイルが生成 | 正常系 |
| OPT-008 | test_combined_patch_file | --patch-fileオプション | 統合パッチファイルが生成 | 正常系 |
| OPT-009 | test_excel_report | --excelオプション | .xlsxファイルが生成 | 正常系 |
| OPT-010 | test_summary_file | --summaryオプション | サマリーファイルが生成 | 正常系 |
| OPT-011 | test_verbose_mode | --verboseオプション | 詳細出力 | 正常系 |
| OPT-012 | test_stats_only | --stats-onlyオプション | ツリー/詳細非表示、統計のみ表示 | 正常系 |
| OPT-013 | test_filter_status_added | --filter-status added | Addedファイルのみコピー | 正常系 |
| OPT-014 | test_filter_status_modified | --filter-status modified | Modifiedファイルのみコピー | 正常系 |
| OPT-015 | test_force_option | --forceオプション（dry-run併用） | 確認なしで続行 | 準正常系 |
| OPT-016 | test_no_tree_option | --no-treeオプション | ツリー表示なし | 正常系 |
| OPT-017 | test_no_details_option | --no-detailsオプション | 詳細表示なし | 正常系 |
| OPT-018 | test_workers_option | --workersオプション | 指定ワーカー数で並列処理 | 準正常系 |
| OPT-019 | test_check_permissions_scripts | -P scriptsオプション | スクリプトの権限変更を検出 | 正常系 |
| OPT-020 | test_check_permissions_all | -P allオプション | 全ファイルの権限変更を検出 | 正常系 |

---

### 5. 日本語パステスト

| テストID | テスト名 | テスト内容 | 期待結果 | 分類 |
|---------|---------|-----------|---------|------|
| JP-001 | test_japanese_directory_names | 日本語ディレクトリ名 | 正しく処理 | 正常系 |
| JP-002 | test_japanese_file_names | 日本語ファイル名 | 正しく処理・コピー | 正常系 |
| JP-003 | test_japanese_nested_path | 深いネストの日本語パス | 正しく処理 | 正常系 |
| JP-004 | test_mixed_japanese_english_path | 日本語と英語混合パス | 正しく処理 | 正常系 |
| JP-005 | test_japanese_in_summary_output | サマリーに日本語表示 | 正しく表示 | 正常系 |
| JP-006 | test_japanese_both_versions | 日本語ファイルの両バージョン | .old/.newが正しく作成 | 正常系 |
| JP-007 | test_japanese_copy_deleted | 日本語ファイルの削除コピー | .deletedが正しく作成 | 正常系 |
| JP-008 | test_japanese_patch_generation | 日本語ファイルのパッチ生成 | パッチに日本語が正しく含まれる | 正常系 |
| JP-009 | test_japanese_excel_report | 日本語パスでExcel生成 | .xlsxが正しく生成 | 正常系 |
| JP-010 | test_japanese_summary_file | 日本語パスでサマリー生成 | サマリーファイルが生成 | 正常系 |
| JP-011 | test_japanese_three_way | 日本語パスで三者間比較 | 正しく比較 | 正常系 |
| JP-012 | test_japanese_content_in_file | ファイル内容に日本語 | 正しく比較・コピー | 正常系 |
| JP-013 | test_hiragana_katakana_kanji_mixed | ひらがな・カタカナ・漢字混合 | 正しく処理 | 準正常系 |
| JP-014 | test_long_japanese_filename | 長い日本語ファイル名 | 正しく処理 | 準正常系 |

---

### 6. エラーハンドリングテスト

| テストID | テスト名 | テスト内容 | 期待結果 | 分類 |
|---------|---------|-----------|---------|------|
| ERR-001 | test_nonexistent_repository | 存在しないリポジトリ | 終了コード1、エラーメッセージ | 異常系 |
| ERR-002 | test_nonexistent_source_ref | 存在しないソースref | 終了コード1、エラーメッセージ | 異常系 |
| ERR-003 | test_nonexistent_target_ref | 存在しないターゲットref | 終了コード1、エラーメッセージ | 異常系 |
| ERR-004 | test_output_exists_without_force | 出力ディレクトリが既存（--forceなし） | 終了コード1、エラーメッセージ | 準正常系 |
| ERR-005 | test_invalid_glob_pattern | 不正なglobパターン | エラー終了 | 異常系 |
| ERR-006 | test_missing_required_args | 必須引数なし | エラー終了、使用方法表示 | 異常系 |
| ERR-007 | test_not_git_repository | Gitリポジトリでないディレクトリ | エラー終了 | 異常系 |

---

### 7. 終了コードテスト

| テストID | テスト名 | テスト内容 | 期待結果 | 分類 |
|---------|---------|-----------|---------|------|
| EXIT-001 | test_exit_code_0_with_differences | 差分あり | 終了コード0 | 正常系 |
| EXIT-002 | test_exit_code_1_on_error | エラー発生 | 終了コード1 | 異常系 |
| EXIT-003 | test_exit_code_2_no_differences | 差分なし | 終了コード2 | 正常系 |
| EXIT-004 | test_exit_code_3_conflicts | コンフリクトあり | 終了コード3 | 正常系 |

---

### 8. 保護ディレクトリテスト（ユニットテストのみ）

**注意**: このテストは実際のシステムディレクトリを誤って削除するリスクがあるため、
結合テストではなくユニットテスト（`src/safety.rs`）で検証します。

| テストID | テスト名 | テスト内容 | 期待結果 | 備考 |
|---------|---------|-----------|---------|------|
| PROT-001 | test_is_protected_path_unix_root | /が保護されるか | 保護される | ユニットテスト |
| PROT-002 | test_is_protected_path_unix_system_dirs | /etc等が保護されるか | 保護される | ユニットテスト |
| PROT-003 | test_is_protected_path_unix_safe_paths | 一時ディレクトリは保護されないか | 保護されない | ユニットテスト |
| PROT-004 | test_validate_output_path_protected | 保護パスへの出力を拒否 | エラーを返す | ユニットテスト |
| PROT-005 | test_validate_output_path_safe | 安全なパスへの出力を許可 | OKを返す | ユニットテスト |

---

### 9. 設定ファイルテスト

| テストID | テスト名 | テスト内容 | 期待結果 | 分類 |
|---------|---------|-----------|---------|------|
| CFG-001 | test_config_file_basic | 基本的な設定ファイル | 設定が読み込まれる | 正常系 |
| CFG-002 | test_config_file_with_exclude | excludeパターン指定 | 除外パターンが適用 | 正常系 |
| CFG-003 | test_cli_overrides_config | CLI引数でオーバーライド | CLI引数が優先 | 正常系 |
| CFG-004 | test_save_config | --save-configオプション | 設定ファイルが保存される | 正常系 |

---

### 10. show_unchangedテスト

| テストID | テスト名 | テスト内容 | 期待結果 | 分類 |
|---------|---------|-----------|---------|------|
| SHOW-001 | test_show_unchanged_option | --show-unchangedオプション | 未変更ファイルが表示 | 正常系 |
| SHOW-002 | test_show_unchanged_short_option | -uオプション | 未変更ファイルが表示 | 正常系 |
| SHOW-003 | test_unchanged_count_in_statistics | 統計に未変更数表示 | 常に未変更数が表示 | 正常系 |

---

### 11. 拡張三者間比較テスト

| テストID | テスト名 | テスト内容 | 期待結果 | 分類 |
|---------|---------|-----------|---------|------|
| EXT3-001 | test_three_way_added_ours | Oursのみ追加 | added-oursとして検出 | 正常系 |
| EXT3-002 | test_three_way_added_theirs | Theirsのみ追加 | added-theirsとして検出 | 正常系 |
| EXT3-003 | test_three_way_deleted_ours | Oursで削除 | deleted-oursとして検出 | 正常系 |
| EXT3-004 | test_three_way_deleted_theirs | Theirsで削除 | deleted-theirsとして検出 | 正常系 |
| EXT3-005 | test_three_way_deleted_both | 両方で削除 | deleted-bothとして検出 | 正常系 |
| EXT3-006 | test_three_way_modify_delete_conflict | 変更と削除の競合 | CONFLICTとして検出 | 準正常系 |
| EXT3-007 | test_three_way_merge_style_ours | --merge-style ours | Ours版のみコピー | 準正常系 |
| EXT3-008 | test_three_way_merge_style_theirs | --merge-style theirs | Theirs版のみコピー | 準正常系 |
| EXT3-009 | test_three_way_conflict_only | --conflict-only | 競合ファイルのみコピー | 準正常系 |

---

### 12. Git ref形式テスト

| テストID | テスト名 | テスト内容 | 期待結果 | 分類 |
|---------|---------|-----------|---------|------|
| REF-001 | test_branch_name_with_slash | ブランチ名にスラッシュ（feature/test） | 正しく処理 | 正常系 |
| REF-002 | test_short_commit_hash | 短縮コミットハッシュ（7文字） | 正しく解決 | 正常系 |
| REF-003 | test_full_commit_hash | 完全コミットハッシュ（40文字） | 正しく解決 | 正常系 |
| REF-004 | test_head_caret_notation | HEAD^表記 | 正しく解決 | 正常系 |
| REF-005 | test_head_tilde_notation | HEAD~N表記 | 正しく解決 | 正常系 |

---

### 13. サマリーファイル内容検証テスト

| テストID | テスト名 | テスト内容 | 期待結果 | 分類 |
|---------|---------|-----------|---------|------|
| SUM-001 | test_summary_contains_header | ヘッダー情報 | タイトル、Repository、Source、Target、Output、Date含む | 正常系 |
| SUM-002 | test_summary_statistics_accuracy | 統計の正確性 | Added/Modified/Deleted数が正しい | 正常系 |
| SUM-003 | test_summary_contains_file_tree | ファイルツリー | File Treeセクション、ファイル名、ステータスタグ含む | 正常系 |
| SUM-004 | test_summary_contains_details | 詳細セクション | Modified Filesセクションとファイル名含む | 正常系 |
| SUM-005 | test_summary_japanese_paths | 日本語パス | 日本語のパス・ファイル名が正しく表示 | 正常系 |
| SUM-006 | test_summary_options_section | オプションセクション | Options、Dry run、Exclude patterns含む | 正常系 |
| SUM-007 | test_summary_no_differences | 差分なし時 | 適切なメッセージ表示 | 準正常系 |

---

### 14. Excelファイル内容検証テスト

**使用クレート**: calamine（Excelファイル読み取り）

| テストID | テスト名 | テスト内容 | 期待結果 | 分類 |
|---------|---------|-----------|---------|------|
| EXCEL-001 | test_excel_has_correct_sheets | シート名の検証 | Summary, File Tree, Detailsシートが存在 | 正常系 |
| EXCEL-002 | test_excel_summary_statistics | Summaryシートの統計 | Added/Modified数が正しい | 正常系 |
| EXCEL-003 | test_excel_file_tree_entries | File Treeシートのエントリ | ファイルとステータスが正しい | 正常系 |
| EXCEL-004 | test_excel_japanese_filenames | 日本語ファイル名 | Excelに日本語が正しく記録 | 正常系 |

---

### 15. パッチファイル内容検証テスト

| テストID | テスト名 | テスト内容 | 期待結果 | 分類 |
|---------|---------|-----------|---------|------|
| PATCH-001 | test_patch_file_unified_format | unified diff形式 | --- a/, +++ b/, @@, -/+行含む | 正常系 |
| PATCH-002 | test_combined_patch_file | 統合パッチファイル | 複数ファイルのパッチが1ファイルに | 正常系 |
| PATCH-003 | test_patch_japanese_content | 日本語内容 | 日本語の差分が正しく表示 | 正常系 |
| PATCH-004 | test_patch_binary_skipped | バイナリスキップ | バイナリファイルはパッチ生成されない | 準正常系 |
| PATCH-005 | test_patch_addition_only | 追加のみ | 追加ファイルが+行として表示 | 正常系 |
| PATCH-006 | test_patch_deletion_only | 削除のみ | 削除内容が-行として表示 | 正常系 |
| PATCH-007 | test_patch_multiple_hunks | 複数ハンク | 複数の変更箇所が別々のハンクとして表示 | 正常系 |
| PATCH-008 | test_patch_hunk_header_format | ハンクヘッダー形式 | @@ -行番号,行数 +行番号,行数 @@ 形式 | 正常系 |
| PATCH-009 | test_patch_subdirectory | サブディレクトリ | パスにディレクトリ構造が含まれる | 正常系 |
| PATCH-010 | test_patch_and_patch_file_together | patch + patch-file併用 | 両方のパッチファイルが生成 | 正常系 |

---

### 16. エッジケーステスト

| テストID | テスト名 | テスト内容 | 期待結果 | 分類 |
|---------|---------|-----------|---------|------|
| EDGE-001 | test_empty_file | 空ファイル | 正常に処理される | 準正常系 |
| EDGE-002 | test_same_size_different_content | 同サイズ異内容 | Modifiedとして検出 | 正常系 |
| EDGE-003 | test_deeply_nested_directories | 深いディレクトリ | 正常に処理される | 正常系 |
| EDGE-004 | test_unicode_russian_filenames | ロシア語ファイル名 | 正常に処理される | 正常系 |
| EDGE-005 | test_exclude_pycache_pattern | __pycache__除外 | 正常に除外される | 正常系 |

---

### 17. シンボリックリンクテスト (Unix only)

| テストID | テスト名 | テスト内容 | 期待結果 | 分類 |
|---------|---------|-----------|---------|------|
| SYM-001 | test_symlink_added_detection | シンボリックリンク追加 | 検出される（スキップまたはリンク） | 準正常系 |
| SYM-002 | test_symlink_deleted_detection | シンボリックリンク削除 | 検出される | 準正常系 |
| SYM-003 | test_broken_symlink_detection | 壊れたシンボリックリンク | エラーなく処理される | 準正常系 |

---

### 18. パーミッションテスト (Unix only)

| テストID | テスト名 | テスト内容 | 期待結果 | 分類 |
|---------|---------|-----------|---------|------|
| PERM-001 | test_permission_check_default_disabled | デフォルト無効 | 権限変更が無視される | 正常系 |
| PERM-002 | test_permission_check_scripts_mode | -P scriptsモード | スクリプト権限変更が検出される | 正常系 |

---

### 19. Excelフォーマット詳細テスト

| テストID | テスト名 | テスト内容 | 期待結果 | 分類 |
|---------|---------|-----------|---------|------|
| EXFMT-001 | test_excel_file_generation | Excelファイル生成 | .xlsxファイルが作成される | 正常系 |
| EXFMT-002 | test_excel_fold_level_option | 折りたたみレベル指定 | 指定レベル以上が折りたたみ | 正常系 |
| EXFMT-003 | test_excel_fold_level_zero_default | デフォルト折りたたみ | レベル0で折りたたみなし | 正常系 |
| EXFMT-004 | test_excel_subdirectory_structure | サブディレクトリ構造 | ディレクトリ階層が正しく記録 | 正常系 |
| EXFMT-005 | test_excel_summary_has_labels | Summaryシートのラベル | Repository, Source, Target等含む | 正常系 |
| EXFMT-006 | test_excel_summary_has_statistics_section | 統計セクション | Added, Modified, Deleted, Total含む | 正常系 |
| EXFMT-007 | test_excel_summary_has_options_section | オプションセクション | Options情報が含まれる | 正常系 |
| EXFMT-008 | test_excel_details_has_header | Detailsヘッダー | ヘッダー行が存在 | 正常系 |
| EXFMT-009 | test_excel_details_sections | Detailsセクション | ステータスごとのセクション | 正常系 |
| EXFMT-010 | test_excel_file_tree_cell_structure | セル構造 | パスとステータスが正しいセルに | 正常系 |

---

### 20. Filter statusテスト

| テストID | テスト名 | テスト内容 | 期待結果 | 分類 |
|---------|---------|-----------|---------|------|
| FILT-001 | test_filter_status_added | --filter-status added | Addedのみコピー | 正常系 |
| FILT-002 | test_filter_status_modified | --filter-status modified | Modifiedのみコピー | 正常系 |
| FILT-003 | test_filter_status_multiple_values | 複数指定 | 指定したステータスのみ | 正常系 |
| FILT-004 | test_no_filter_status_when_not_specified | 未指定時 | 全ステータス含む | 正常系 |
| FILT-005 | test_excel_contains_filter_status_option | Excelにfilter-status | オプションが記録される | 正常系 |
| FILT-006 | test_excel_no_filter_status_when_not_specified | 未指定でExcel | filter-statusなし | 正常系 |
| FILT-007 | test_summary_contains_filter_status_option | サマリーにfilter-status | オプションが記録される | 正常系 |

---

### 21. 統計テスト

| テストID | テスト名 | テスト内容 | 期待結果 | 分類 |
|---------|---------|-----------|---------|------|
| STATS-001 | test_unchanged_count_always_shown | 未変更数常時表示 | Unchangedが統計に含まれる | 正常系 |
| STATS-002 | test_unchanged_count_with_many_files | 多数ファイル | 正しい未変更数 | 正常系 |
| STATS-003 | test_unchanged_count_same_with_or_without_option | -u有無で同数 | 統計の未変更数は同じ | 正常系 |
| STATS-004 | test_statistics_categories_sum | カテゴリ合計 | 各カテゴリの合計がTotal | 正常系 |
| STATS-005 | test_statistics_with_filter | フィルタ時統計 | フィルタ後も正しい統計 | 正常系 |
| STATS-006 | test_total_equals_all_unique_paths | Total=ユニークパス数 | 総数が一意パス数と一致 | 正常系 |

---

### 22. Tree表示テスト

| テストID | テスト名 | テスト内容 | 期待結果 | 分類 |
|---------|---------|-----------|---------|------|
| TREE-001 | test_console_output_contains_box_drawing_chars | 罫線文字 | └├─│が含まれる | 正常系 |
| TREE-002 | test_summary_file_contains_box_drawing_chars | サマリーファイル罫線 | └├─│が含まれる | 正常系 |
| TREE-003 | test_box_drawing_chars_at_different_depths | 深さ別罫線 | ネスト時も正しく表示 | 正常系 |
| TREE-004 | test_tree_structure_formatting | ツリー構造 | インデントが正しい | 正常系 |
| TREE-005 | test_three_way_tree_contains_box_drawing_chars | 三者間比較罫線 | 三者間でも罫線表示 | 正常系 |

---

## 終了コード一覧

| 終了コード | 意味 |
|-----------|------|
| 0 | 差分あり（正常終了） |
| 1 | エラー発生 |
| 2 | 差分なし |
| 3 | コンフリクトあり（三者間比較時） |

---

## テストデータ構造

各テストでは、一時ディレクトリにGitリポジトリを動的に作成します。

```
temp_dir/
├── test_repo/           # テスト用Gitリポジトリ
│   ├── .git/
│   ├── unchanged.txt    # 変更なしファイル
│   ├── modified.txt     # 変更ありファイル
│   ├── added.txt        # 追加ファイル（targetブランチ）
│   ├── deleted.txt      # 削除ファイル（targetブランチ）
│   └── subdir/
│       └── nested.txt
└── output/              # 出力先ディレクトリ
```

### テスト用リポジトリの作成手順

1. 一時ディレクトリを作成
2. `git init` でリポジトリを初期化
3. 初期ファイルを作成して `git add` → `git commit`（sourceコミット）
4. ファイルを変更/追加/削除して `git add` → `git commit`（targetコミット）
5. テスト実行
6. 一時ディレクトリを削除

---

## 実行方法

```bash
# すべての結合テストを実行
cargo test --test integration_tests

# 特定のテストを実行
cargo test --test integration_tests test_two_way_

# 日本語パステストのみ
cargo test --test integration_tests japanese

# 準正常系テストのみ
cargo test --test integration_tests semi_normal

# 詳細出力付きで実行
cargo test --test integration_tests -- --nocapture

# テスト結果を記録
cargo test --test integration_tests 2>&1 | tee test_output.txt
```

---

### 23. 不具合修正確認テスト

| テストID | テスト名 | テスト内容 | 期待結果 | 分類 |
|---------|---------|-----------|---------|------|
| BUGFIX-001 | test_summary_file_no_ansi_escape_codes | サマリーファイルにANSIエスケープコードが含まれないこと | ファイル内に\x1b[、[32m、[0m等が含まれない | 正常系 |
| BUGFIX-002 | test_summary_file_status_alignment | サマリーファイルのステータス位置が整列されていること | すべての[added]等のタグが同じ列位置に表示される | 正常系 |
| BUGFIX-003 | test_excel_file_has_borders | Excelファイルの罫線が正しく設定されていること | File TreeシートとDetailsシートが存在し、データが含まれる | 正常系 |
| BUGFIX-004 | test_summary_file_status_alignment_with_japanese | 日本語ファイル名でもステータス位置が整列されること | ANSIエスケープコードが含まれず、[added]タグが正しく表示される | 正常系 |

---

### 24. 三者間グループキーワードテスト

| テストID | テスト名 | テスト内容 | 期待結果 | 分類 |
|---------|---------|-----------|---------|------|
| GRP-001 | test_group_keyword_exclusion_added | `--filter-status all,^added`で^addedグループ除外 | added-ours, added-theirs, added-both-same, added-both-diffが除外される | 正常系 |
| GRP-002 | test_group_keyword_exclusion_deleted | `--filter-status all,^deleted`で^deletedグループ除外 | deleted-ours, deleted-theirs, deleted-bothが除外される | 正常系 |
| GRP-003 | test_group_keyword_exclusion_modified | `--filter-status all,^modified`で^modifiedグループ除外 | ours-only, theirs-only, both-same, conflictが除外される | 正常系 |
| GRP-004 | test_group_keyword_exclusion_conflicts | `--filter-status all,^conflicts`で^conflictsグループ除外 | conflict, added-both-diff, modify-delete, delete-modifyが除外される | 正常系 |
| GRP-005 | test_group_keyword_inclusion_added | `--filter-status added`でaddedグループ包含 | added-*のみが表示され、他は除外される | 正常系 |
| GRP-006 | test_group_keyword_multiple | `--filter-status added,deleted`で複数グループ | added-*とdeleted-*のみが表示される | 正常系 |

---

### 25. 三者間File Tree整列テスト

| テストID | テスト名 | テスト内容 | 期待結果 | 分類 |
|---------|---------|-----------|---------|------|
| ALIGN-001 | test_three_way_summary_file_alignment | 三者間Summaryファイル整列 | 異なる長さのパスでもインジケータが同じ位置に揃う | 正常系 |
| ALIGN-002 | test_three_way_summary_file_alignment_japanese | 三者間日本語ファイル名整列 | 日本語ファイル名でも表示幅でインジケータが揃う | 正常系 |
| ALIGN-003 | test_three_way_file_tree_alignment | 三者間File Tree整列 | 異なる深さのファイルでもインジケータが同じ位置に揃う | 正常系 |

---

### 26. ExcelファイルTree ディレクトリ分割形式テスト

| テストID | テスト名 | テスト内容 | 期待結果 | 分類 |
|---------|---------|-----------|---------|------|
| EXCEL-TREE-001 | test_excel_file_tree_directory_split_two_way | 二者間比較でのFileTreeディレクトリ分割 | パス各コンポーネントが別々のセルに配置される（src/, lib/, utils.rs等） | 正常系 |
| EXCEL-TREE-002 | test_excel_file_tree_directory_split_three_way | 三者間比較でのFileTreeディレクトリ分割 | パス各コンポーネントが別々のセルに配置される | 正常系 |
| EXCEL-TREE-003 | test_excel_file_tree_status_column_position_two_way | 二者間比較でのStatus列位置検証 | Status列がパスコンポーネントの後ろ（max_depth + 1）に配置され、ファイル名と重ならない | 正常系 |
| EXCEL-TREE-004 | test_excel_file_tree_status_column_position_three_way | 三者間比較でのStatus列位置検証 | Status列がパスコンポーネントの後ろ（max_depth + 1）に配置され、ファイル名と重ならない | 正常系 |

---

### 27. Excelファイル グルーピング機能テスト

| テストID | テスト名 | テスト内容 | 期待結果 | 分類 |
|---------|---------|-----------|---------|------|
| EXCEL-GROUP-001 | test_excel_fold_level_two_way | 二者間比較での--excel-fold-levelオプション | 指定深さ以上の行がグルーピングされる | 正常系 |
| EXCEL-GROUP-002 | test_excel_fold_level_three_way | 三者間比較での--excel-fold-levelオプション | 指定深さ以上の行がグルーピングされる | 正常系 |
| EXCEL-GROUP-003 | test_excel_default_fold_level | デフォルトのfold-level（グルーピングなし） | --excel-fold-level未指定時はグルーピングなし | 正常系 |

---

## 変更履歴

| 日付 | バージョン | 変更内容 |
|------|-----------|---------|
| 2026-01-21 | 1.0 | 初版作成 |
| 2026-01-21 | 1.1 | パッチファイルテスト (PATCH-001〜010)、エッジケーステスト (EDGE-001〜005)、シンボリックリンクテスト (SYM-001〜003)、パーミッションテスト (PERM-001〜002)、Excelフォーマット詳細テスト (EXFMT-001〜010)、Filter statusテスト (FILT-001〜007)、統計テスト (STATS-001〜006)、Tree表示テスト (TREE-001〜005) を追加 |
| 2026-01-21 | 1.2 | 不具合修正確認テスト (BUGFIX-001〜004) を追加：サマリーファイルのANSIエスケープコード問題、ステータス位置整列問題、Excel罫線問題の修正確認 |
| 2026-01-21 | 1.3 | 三者間グループキーワードテスト (GRP-001〜006)、三者間File Tree整列テスト (ALIGN-001〜003) を追加 |
| 2026-01-22 | 1.4 | ExcelファイルTree ディレクトリ分割形式テスト (EXCEL-TREE-001〜002)、Excelファイル グルーピング機能テスト (EXCEL-GROUP-001〜003) を追加 |
| 2026-01-22 | 1.5 | ExcelファイルTree Status列位置検証テスト (EXCEL-TREE-003〜004) を追加：Status列がパスコンポーネントと重ならないことを検証 |
| 2026-01-22 | 1.6 | 三者間比較ファイル出力形式テスト (THREE-COPY-001〜005) を追加：ステータスサフィックス形式のファイル出力検証 |
