project('salam', 'rust')

cargo = find_program('cargo')

buildtype = get_option('buildtype')
cargo_mode = buildtype.startswith('debug') ? [] : ['--release']

custom_target(
  'salam',
  build_by_default: true,
  output: ['dummy-output'],
  command: [
    cargo,
    'build',
    '--verbose',
    '--manifest-path', meson.project_source_root() / 'Cargo.toml',
  ]
  + cargo_mode,
  console: true,
)

meson.add_install_script(
  meson.project_source_root() / 'scripts/install-cargo-bin.sh',
  'salam',
  (cargo_mode.length() == 0 ? 'debug' : 'release'),
)

install_data('data/salam.desktop', install_dir: get_option('datadir') / 'applications')

install_data(
  'data/salam.png',
  install_dir: get_option('datadir') / 'icons' / 'hicolor' / '512x512' / 'apps',
)
