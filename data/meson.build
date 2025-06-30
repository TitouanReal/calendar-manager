blueprints = custom_target(
    'blueprints',
    input: files(
        'resources/ui/calendar_manager_dialog/calendar_details_page.blp',
        'resources/ui/calendar_manager_dialog/calendar_manager_dialog.blp',
        'resources/ui/calendar_manager_dialog/calendar_row.blp',
        'resources/ui/calendar_manager_dialog/collection_row.blp',
        'resources/ui/calendar_manager_dialog/collections_list.blp',
        'resources/ui/window.blp',
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

desktop_file = i18n.merge_file(
    input: 'io.gitlab.TitouanReal.CalendarManager.desktop.in',
    output: 'io.gitlab.TitouanReal.CalendarManager.desktop',
    type: 'desktop',
    po_dir: '../po',
    install: true,
    install_dir: get_option('datadir') / 'applications',
)

desktop_utils = find_program('desktop-file-validate', required: false)
if desktop_utils.found()
    test('Validate desktop file', desktop_utils, args: [desktop_file])
endif

appstream_file = i18n.merge_file(
    input: 'io.gitlab.TitouanReal.CalendarManager.metainfo.xml.in',
    output: 'io.gitlab.TitouanReal.CalendarManager.metainfo.xml',
    po_dir: '../po',
    install: true,
    install_dir: get_option('datadir') / 'metainfo',
)

appstreamcli = find_program('appstreamcli', required: false, disabler: true)
test(
    'Validate appstream file',
    appstreamcli,
    args: ['validate', '--no-net', '--explain', appstream_file],
)

install_data(
    'io.gitlab.TitouanReal.CalendarManager.gschema.xml',
    install_dir: get_option('datadir') / 'glib-2.0' / 'schemas',
)

compile_schemas = find_program('glib-compile-schemas', required: false, disabler: true)
test(
    'Validate schema file',
    compile_schemas,
    args: ['--strict', '--dry-run', meson.current_source_dir()],
)

service_conf = configuration_data()
service_conf.set('bindir', get_option('prefix') / get_option('bindir'))
configure_file(
    input: 'io.gitlab.TitouanReal.CalendarManager.service.in',
    output: 'io.gitlab.TitouanReal.CalendarManager.service',
    configuration: service_conf,
    install_dir: get_option('datadir') / 'dbus-1' / 'services',
)

subdir('icons')
