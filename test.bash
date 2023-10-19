#!/bin/bash

function send_request() {
  VAR_SERVER="http://localhost:8080/random"
  VAR_QUERY="?format=$2"
  echo -ne "-------------\nRequest: $1\nResponse ($2): "
  curl --header "Content-Type: application/json" --request POST --data "$1" "${VAR_SERVER}${VAR_QUERY}"
  echo ""
}

send_request '{"distribution":"uniform","parameters":{"start":-100,"end":100}}' json 
send_request '{"distribution":"shuffle","parameters":{"data":"MTIzNDU2Nzg5MA=="}}' json
send_request '{"distribution":"color","parameters":{"from":"black","to":"#EC670F"}}' cbor
#unsupported parameter, error
send_request '{"distribution":"gamma","parameters":{"shape":2.0,"scale":5.0}}' json
#unsupported format, error
send_request '{"distribution":"uniform","parameters":{"start":-100,"end":100}}' xml 

