project('calendar-manager', 'rust',
          version: '0.1.0',
    meson_version: '>= 1.0.0',
  default_options: [ 'warning_level=2', 'werror=false', ],
)

dependency('gtk4', version: '>= 4.18')
dependency('libadwaita-1', version: '>= 1.7')
dependency('tinysparql-3.0', version: '>= 3.9')

i18n = import('i18n')
gnome = import('gnome')

subdir('data')
subdir('src')
subdir('po')

gnome.post_install(
     glib_compile_schemas: true,
    gtk_update_icon_cache: true,
  update_desktop_database: true,
)
