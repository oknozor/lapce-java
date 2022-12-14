xdg_data_dir :=  "$HOME/.local/share"
plugin_dir := "plugins/oknozor.lapce-java"

build:
    cargo make
    
install-stable: build
    mkdir -p {{xdg_data_dir}}/lapce-stable/{{plugin_dir}}/bin
    yes | cp -i bin/lapce-java.wasm {{xdg_data_dir}}/lapce-stable/{{plugin_dir}}/bin
    yes | cp -i volt.toml {{xdg_data_dir}}/lapce-stable/{{plugin_dir}}/
    yes | cp -i JavaIcon.svg {{xdg_data_dir}}/lapce-stable/{{plugin_dir}}/
    rm -rd {{xdg_data_dir}}/lapce-stable/{{plugin_dir}}/jdt-language-server-latest || true

install-debug: build
    mkdir -p {{xdg_data_dir}}/lapce-debug/{{plugin_dir}}/bin
    yes | cp -i bin/lapce-java.wasm {{xdg_data_dir}}/lapce-debug/{{plugin_dir}}/bin
    yes | cp -i volt.toml {{xdg_data_dir}}/lapce-debug/{{plugin_dir}}
    yes | cp -i JavaIcon.svg {{xdg_data_dir}}/lapce-stable/{{plugin_dir}}/
    rm -rd {{xdg_data_dir}}/lapce-debug/{{plugin_dir}}/jdt-language-server-latest || true
