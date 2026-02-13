# html/browsers/browsing-the-web/back-forward-cache/service-worker-clients-matchall.https.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/back-forward-cache/service-worker-clients-matchall.https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta name="timeout" content="long">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/common/utils.js"></script>
<script src="/common/dispatcher/dispatcher.js"></script>
<script src="resources/helper.sub.js"></script>
<script src="/service-workers/service-worker/resources/test-helpers.sub.js"></script>
<script>
promise_test(async t => {
  // Register a service worker and make this page controlled.
  const workerUrl =
      'resources/service-worker.js?pipe=header(Service-Worker-Allowed,../)';
  const registration =
      await service_worker_unregister_and_register(t, workerUrl, './');
  t.add_cleanup(_ => registration.unregister());
  await wait_for_state(t, registration.installing, 'activated');
  const controllerChanged = new Promise(
      resolve => navigator.serviceWorker.oncontrollerchange = resolve);
  await claim(t, registration.active);
  await controllerChanged;

  const pageA = new RemoteContext(token());
  const pageB = new RemoteContext(token());

  const urlA = location.origin + executorPath + pageA.context_id;
  const urlB = originCrossSite + executorPath + pageB.context_id;

  // Open `urlA`.
  window.open(urlA, '_blank', 'noopener');
  await pageA.execute_script(waitForPageShow);

  // Get Clients.matchAll() and check whether `pageA` is controlled.
  // Actual `assert_*()` is called after `assert_bfcached()` below.
  const clients1 = await (await fetch('/get-clients-matchall')).json();
  const controlled1 = await pageA.execute_script(
      () => (navigator.serviceWorker.controller !== null));

  // Navigate to `urlB` and get Clients.matchAll() when `urlA` is in BFCache.
  await pageA.execute_script(
    (url) => prepareNavigation(() => {
      location.href = url;
    }),
    [urlB]);
  await pageB.execute_script(waitForPageShow);
  const clients2 = await (await fetch('/get-clients-matchall')).json();

  // Back navigate and check whether the page is restored from BFCache.
  await pageB.execute_script(
    () => {
      prepareNavigation(() => { history.back(); });
    }
  );
  await pageA.execute_script(waitForPageShow);
  await assert_bfcached(pageA);

  // Get Clients.matchAll() and check whether `pageA` is controlled.
  const clients3 = await (await fetch('/get-clients-matchall')).json();
  const controlled3 = await pageA.execute_script(
      () => (navigator.serviceWorker.controller !== null));

  // Clients.matchAll() should not list `urlA` when it is in BFCache.
  assert_true(clients1.indexOf(urlA) >= 0,
      '1: matchAll() before navigation');
  assert_true(clients2.indexOf(urlA) < 0,
      '2: matchAll() before back navigation');
  assert_true(clients3.indexOf(urlA) >= 0,
      '3: matchAll() after back navigation');

  // `pageA` should be controlled before/after BFCached.
  assert_true(controlled1,
    'pageA should be controlled before BFCached');
  assert_true(controlled3,
    'pageA should be controlled after restored');
}, 'Clients.matchAll() should not list pages in BFCache');
</script>
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
  "source_name": "html/browsers/browsing-the-web/back-forward-cache/service-worker-clients-matchall.https.html"
}
```
