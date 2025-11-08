email: cd backend && cargo watch -w email-service/src -w proto -x "run --package email-service"
auth: bash -c "scripts/wait-for-port.sh localhost 9001 60 && cd backend && cargo watch -w auth-service/src -w shared-types/src -w proto -x 'run --package auth-service'"
settings: bash -c "scripts/wait-for-port.sh localhost 8000 60 && cd backend && cargo watch -w settings-service/src -w shared-types/src -w proto -x 'run --package settings-service'"
transaction: bash -c "scripts/wait-for-port.sh localhost 8000 60 && cd backend && cargo watch -w transaction-service/src -w shared-types/src -w proto -x 'run --package transaction-service'"
frontend: cd frontend && npm run dev
