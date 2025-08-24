project('salam', 'rust')

cargo = find_program('cargo')

buildtype = get_option('buildtype')
cargo_mode = buildtype.startswith('debug') ? [] : ['--release']

custom_target(
  'salam',
  build_by_default: true,
  output: ['salam'],
  command: [cargo, 'build']
  + cargo_mode
  + ['--manifest-path', meson.project_source_root() / 'Cargo.toml'],

  depend_files: [
    meson.project_source_root() / 'Cargo.toml',
    meson.project_source_root() / 'ui/welcome.slint',
  ],
  install: true,
  install_dir: get_option('bindir'),
)

install_data('data/salam.desktop', install_dir: get_option('datadir') / 'applications')

install_data(
  'data/salam.png',
  install_dir: get_option('datadir') / 'icons' / 'hicolor' / '512x512' / 'apps',
)
