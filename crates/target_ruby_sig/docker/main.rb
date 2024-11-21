require 'json'
require_relative 'gen/jetted_e2e'

$stdout.sync = true
$stdin.each do |line|
  value = JettedE2E::MAIN.from_json(JSON.parse(line))
  puts JSON.generate(value.to_json)
end
