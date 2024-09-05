# Private State Token Demo Monkeypatch

Google's unofficial [Private State Token](https://developers.google.com/privacy-sandbox/protections/private-state-tokens) demo is broken:

- [glitch demo](https://private-state-token-demo.glitch.me/)
- [github repo](https://github.com/JackJey/private-state-token-demo)

This is a Fastly Compute service that "monkeypatches" the demo and serves at a different URL.

- Try it: [pst-demo.foote.dev](https://pst-demo.foote.dev)

This is a hack that may break in the future. I don't plan to maintain it.

# Fixes

- Adds missing `Permission-Policy` headers
- Serves a working version of `issuer.js`
    - The version live in the demo is broken; looks like a leftover debug try/catch
    - This version serves the old working version of `issuer.js` from the github repo
- Changes the unix timestamp in the Chrome key commitment argument to be in the future (2025 Sept 5)

# Other stuff

- Transforms hostnames etc. to make the proxying work
