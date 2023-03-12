# conftube

A social network evolving around tech conference videos.

## development

First, the backend needs some configuration: copy `backend/.env.dist` to `backend/.env` and insert your credentials for Auth0 and the YouTube API.

Conftube consists of two parts: the backend can be started with running `npm run dev` in `backend`, the frontend can be
started running `npm run start` and will proxy requests to the backend. Just go to http://localhost:3000 in your browser.

Further instructions can be found in [/backend](/backend) and [/frontend](/frontend) respectively.
