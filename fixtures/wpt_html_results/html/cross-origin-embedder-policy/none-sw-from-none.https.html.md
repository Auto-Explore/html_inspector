# html/cross-origin-embedder-policy/none-sw-from-none.https.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/cross-origin-embedder-policy/none-sw-from-none.https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<script src="/common/get-host-info.sub.js"></script>
<script src="/service-workers/service-worker/resources/test-helpers.sub.js"></script>
<script>
const SCOPE = new URL(location.href).pathname;
const SCRIPT =
  'resources/sw.js?' +
  `pipe=header(service-worker-allowed,${SCOPE})`;

function remote(path) {
  const REMOTE_ORIGIN = get_host_info().HTTPS_REMOTE_ORIGIN;
  return new URL(path, REMOTE_ORIGIN + '/html/cross-origin-embedder-policy/');
}

promise_test(async (t) => {
  const reg = await service_worker_unregister_and_register(t, SCRIPT, SCOPE);
  add_completion_callback(() => {
      reg.unregister();
  });
  await new Promise(resolve => {
    navigator.serviceWorker.addEventListener('controllerchange', resolve);
  });
}, 'setting up');

promise_test(async (t) => {
  await fetch('resources/nothing-same-origin-corp.txt', {mode: 'no-cors'});
}, 'making a same-origin request for CORP: same-origin');

promise_test(async (t) => {
  await fetch('/common/blank.html', {mode: 'no-cors'});
}, 'making a same-origin request for no CORP');

promise_test(async (t) => {
  await fetch('resources/nothing-cross-origin-corp.js', {mode: 'no-cors'});
}, 'making a same-origin request for CORP: cross-origin');

promise_test(async (t) => {
  await promise_rejects_js(
    t, TypeError,
    fetch(remote('resources/nothing-same-origin-corp.txt'), {mode: 'no-cors'}));
}, 'making a cross-origin request for CORP: same-origin');

promise_test(async (t) => {
  await fetch(remote('/common/blank.html'), {mode: 'no-cors'});
}, 'making a cross-origin request for no CORP');

promise_test(async (t) => {
  await fetch(
    remote('resources/nothing-cross-origin-corp.js'),
    {mode: 'no-cors'});
}, 'making a cross-origin request for CORP: cross-origin');

promise_test(async (t) => {
  await promise_rejects_js(
    t, TypeError,
    fetch(remote('resources/nothing-same-origin-corp.txt?passthrough'),
      {mode: 'no-cors'}));
}, 'making a cross-origin request for CORP: same-origin [PASS THROUGH]');

promise_test(async (t) => {
  await fetch(remote('/common/blank.html?passthrough'), {mode: 'no-cors'});
}, 'making a cross-origin request for no CORP [PASS THROUGH]');

promise_test(async (t) => {
  await fetch(
    remote('resources/nothing-cross-origin-corp.js?passthrough'),
    {mode: 'no-cors'});
}, 'making a cross-origin request for CORP: cross-origin [PASS THROUGH]');

promise_test(async (t) => {
  await promise_rejects_js(
    t, TypeError, fetch(remote('/common/blank.html'), {mode: 'cors'}));
}, 'making a cross-origin request with CORS without ACAO');

promise_test(async (t) => {
  const URL = remote(
    '/common/blank.html?pipe=header(access-control-allow-origin,*)');
  await fetch(URL, {mode: 'cors'});
}, 'making a cross-origin request with CORS');

promise_test(async (t) => {
  const URL = remote('/fetch/api/resources/preflight.py?allow_headers=hoge');
  await fetch(URL, {mode: 'cors', headers: {'hoge': 'fuga'}});
}, 'making a cross-origin request with CORS-preflight');
</script>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/cross-origin-embedder-policy/none-sw-from-none.https.html"
}
```
