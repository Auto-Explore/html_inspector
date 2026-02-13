# html/cross-origin-opener-policy/coop-navigate-same-origin-csp-sandbox.html

Counts:
- errors: 1
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/cross-origin-opener-policy/coop-navigate-same-origin-csp-sandbox.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<script src="/common/get-host-info.sub.js"></script>
<script src="/common/utils.js"></script>
<script src="/common/dispatcher/dispatcher.js"></script>
<script src="./resources/common.js"></script>
<script>

const executor_path = '/common/dispatcher/executor.html?pipe=';

const https_origin = get_host_info().HTTPS_ORIGIN;
const coop_same_origin =
    '|header(Cross-Origin-Opener-Policy,same-origin)';
const csp_sandbox =
    '|header(Content-Security-Policy, sandbox allow-scripts)';

promise_test(async test => {
  const driver_token = token();

  // 1. Start from a COOP:same-origin document.
  const opener_token = token();
  const opener_url = https_origin + executor_path + coop_same_origin +
    `&uuid=${opener_token}`;
  const w = window.open(opener_url);
  add_completion_callback(() => w.close());

  // 2. It opens a popups, and then navigates the popup toward a same-origin
  // COOP:same-origin document with CSP:sandbox
  const openee_token = token();
  const openee_url = https_origin + executor_path + coop_same_origin +
    csp_sandbox + `&uuid=${openee_token}`;
  send(opener_token, `
    openee = window.open("${openee_url}");
  `);
  add_completion_callback(() => send(openee_token, "close()"));

  // Because of CSP:sandbox, the popup is not considered same-origin with
  // its openee. Check the openee/opener relationship is now closed.
  send(openee_token, `
    if (opener)
      send("${driver_token}", "Error: have opener");
    else
      send("${driver_token}", "Success: no opener");
  `);
  assert_equals(await receive(driver_token), "Success: no opener");

  // Technically, the opener's "openee" WindowProxy should appear as closed at
  // this time. The popup loaded a new document, and at least two fetch requests
  // were made. This is more than enough. However, in theory, there is nothing
  // to guarantee we can observe "openee.close". Wait a bit to ensure this will
  // never flake.
  await new Promise(r => test.step_timeout(r, 1000));

  send(opener_token, `
    if (openee.closed)
      send("${driver_token}", "Success: openee closed");
    else
      send("${driver_token}", "Error: can still access openee");
  `);
  assert_equals(await receive(driver_token), "Success: openee closed");
});

</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.start_tag_without_doctype",
      "message": "Start tag seen without seeing a doctype first. Expected “<!DOCTYPE html>”.",
      "severity": "Error",
      "span": {
        "byte_end": 38,
        "byte_start": 0,
        "col": 1,
        "line": 1
      }
    },
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
  "source_name": "html/cross-origin-opener-policy/coop-navigate-same-origin-csp-sandbox.html"
}
```
