# KAIST auto login

KAIST auto login web extension and server.

## How does it work?

This works by email authentication.

1. The server would generate email; ex: `kaist-auth-abcdef@example.com`
2. User should set auto-forward email to generated email
   - recommend to use filter (from: `iamps@kaist.ac.kr`)
3. Then the extension would fill in the code