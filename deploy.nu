let user = "shaun"
let pi_url = "192.168.86.38"
let address = $"($user)@($pi_url)"

scp -r .\target\aarch64-unknown-linux-gnu\debug\rasponic $"($address):/tmp/"
open .\tools\run.sh | ssh $address 'bash -s' $in &
