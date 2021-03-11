# NFT Backend
The NFT Marketplace Backend Application


## Development

To run local:
```bash
cargo +nightly run
```
Visit http://localhost:8000/ in your browser.


## Deployment

```bash
heroku login
heroku git:remote -a iota-nft
git push heroku main:master      
```
