blueprints = custom_target(
  'blueprints',
  input: files(
    # Calendar Manager Dialog
    'ui/calendar_manager_dialog/calendar_creation_dialog.blp',
    'ui/calendar_manager_dialog/calendar_details_page.blp',
    'ui/calendar_manager_dialog/calendar_manager_dialog.blp',
    'ui/calendar_manager_dialog/calendar_row.blp',
    'ui/calendar_manager_dialog/collection_row.blp',
    'ui/calendar_manager_dialog/collections_list.blp',

    # Create Event Dialog
    'ui/create_event_dialog/calendar_combo_row/calendar_combo_row.blp',
    'ui/create_event_dialog/calendar_combo_row/calendar_combo_row_header.blp',
    'ui/create_event_dialog/calendar_combo_row/calendar_combo_row_item.blp',
    'ui/create_event_dialog/calendar_combo_row/calendar_combo_row_list_item.blp',
    'ui/create_event_dialog/create_event_dialog.blp',
    'ui/search_dialog/event_row.blp',

    # Search Dialog
    'ui/search_dialog/search_dialog.blp',

    'ui/window.blp',
  ),
  output: '.',
  command: [
    find_program('blueprint-compiler'),
    'batch-compile',
    '@OUTPUT@',
    '@CURRENT_SOURCE_DIR@',
    '@INPUT@',
  ],
)

gnome.compile_resources(
  meson.project_name(),
  'resources.gresource.xml',
  gresource_bundle: true,
  dependencies: blueprints,
  install: true,
  install_dir: pkgdatadir,
)
