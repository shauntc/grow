let user = "shaun"
let pi_url = "192.168.86.38"
let address = $"($user)@($pi_url)"
ssh $address 'pkill -f rasponic'