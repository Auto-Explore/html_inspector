# html/cross-origin-opener-policy/coop-same-origin-allow-popups-document-write.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/cross-origin-opener-policy/coop-same-origin-allow-popups-document-write.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/common/get-host-info.sub.js"></script>
<script src="/common/utils.js"></script>
<script src="/common/dispatcher/dispatcher.js"></script>
<script>

/*
  Regression test for: https://crbug.com/1216244
  From a window using Cross-Origin-Opener-Policy:same-origin-allow-popup, open
  a new blank window and navigate it cross-origin using document.write and a
  meta refresh. The openee/opener relationship must hold.
*/

const executor_path = '/common/dispatcher/executor.html?pipe=';
const coep_soap =
  "|header(Cross-Origin-Opener-Policy,same-origin-allow-popups)";
const same_origin = get_host_info().HTTPS_ORIGIN;
const cross_origin = get_host_info().HTTPS_REMOTE_ORIGIN;

promise_test(async t => {
  // This window:
  const this_window_token = token();

  // The opener, using COEP:same-origin-allow-popups:
  const opener_token = token();
  const opener_url = same_origin + executor_path + coep_soap +
    `&uuid=${opener_token}`;
  const opener = window.open(opener_url);

  // Open a blank window, then use document.write and a meta refresh to navigate
  // cross-origin.
  const openee_token = token();
  const openee_url = cross_origin + executor_path + `&uuid=${openee_token}`;
  send(opener_token, `
    openee = window.open();
    openee.document.write(\`
      <meta http-equiv="refresh" content="0; url=${openee_url}">
    \`);
    openee.document.close();
  `);

  // Check the openee is loaded without access to the opener.
  send(openee_token, `
    send("${this_window_token}", opener == null)
  `);
  assert_equals(await receive(this_window_token), "true", "opener == null");

  // To get the state of the openee reflected into the opener's process, waiting
  // for the openee' document to load and the various fetch() with the
  // dispatcher should be largely enough. However these aren't causal guarantee.
  // So wait a bit to be sure:
  await new Promise(r => t.step_timeout(r, 1000));

  // Check the opener see the openee as 'closed' after the navigation.
  send(opener_token, `
    send("${this_window_token}", openee.closed)
  `);
  assert_equals(await receive(this_window_token), "true", "openee.closed");
});
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
  "source_name": "html/cross-origin-opener-policy/coop-same-origin-allow-popups-document-write.html"
}
```
