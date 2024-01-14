const user = "shaun"
const pi_url = "192.168.86.49"
const pi_ping_url = "pi-grow.local"
const bin_name = "rasponic"

def make_address [] {
    $"($user)@($pi_url)"
}

export def pi_ping [] {
    ping -4 $pi_ping_url
}

export def r_dev [] {
    r_build
    r_upload
    r_run
}

export def r_build [] {
    let target_arch = "aarch64-unknown-linux-gnu"
    echo $"Building for ($target_arch)"
    cross build --target $target_arch
}

export def r_upload [] {
    let file_name = $"./target/aarch64-unknown-linux-gnu/debug/($bin_name)"
    let target_location = $"(make_address):/tmp/"

    echo $"Uploading ($file_name) to ($target_location)"
    scp $file_name $target_location
    echo "Upload complete"
}

export def r_run [] {
    let file_path = $"/tmp/($bin_name)"
    let script = $"chmod +x ($file_path) & ($file_path)"
    echo $"adding execute permissions to ($file_path) and executing it"
    ssh (make_address) 'bash -s' $script &
}

export def r_kill [] {
    ssh (make_address) 'pkill -f rasponic'
}