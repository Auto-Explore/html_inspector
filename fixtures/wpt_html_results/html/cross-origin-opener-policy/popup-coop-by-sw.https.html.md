# html/cross-origin-opener-policy/popup-coop-by-sw.https.html

Counts:
- errors: 1
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/cross-origin-opener-policy/popup-coop-by-sw.https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<meta name="timeout" content="long">
<meta name="variant" content="?1-4">
<meta name="variant" content="?5-last">
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<script src="/common/subset-tests.js"></script>
<script src="/service-workers/service-worker/resources/test-helpers.sub.js"></script>
<script src="/common/utils.js"></script>
<script src="/common/dispatcher/dispatcher.js"></script>
<script src="/common/get-host-info.sub.js"></script>
<script>

const executor_path = '/common/dispatcher/executor.html?pipe=';
const executor_service_worker_path = '/common/dispatcher/executor-service-worker.js?pipe=';

const coop_header = `|header(Cross-Origin-Opener-Policy,same-origin)`;
const coep_header = `|header(Cross-Origin-Embedder-Policy,require-corp)`;

const https_origin = get_host_info().HTTPS_ORIGIN;

const swap_browsing_context_group = true;
const keep_browsing_context_group = false;

const opener_basic = "";
const opener_coi = coop_header + coep_header;

const sw_basic = "";
const sw_coi = coop_header + coep_header;

const openee_basic = {
  'content-type': 'text/html',
};
const openee_coi = {
  'content-type': 'text/html',
  'cross-origin-embedder-policy': 'require-corp',
  'cross-origin-opener-policy': 'same-origin',
};

// A document opens a popup. The popup's document is served from a synthetic
// response by a ServiceWorker. Check how cross-origin isolation works in this
// case.
const popupCoopBySwTest = (
  description,
  // Test parameters:
  opener_headers,
  openee_headers,
  sw_headers,
  // Test expectations:
  expected_browing_context_group
) => {
  subsetTest(promise_test, async test => {
    const driver_token = token();

    // 1. Create the opener.
    const opener_token = token();
    const opener_url = https_origin + executor_path + opener_headers +
      `&uuid=${opener_token}`;
    const opener_window = window.open(opener_url);
    test.add_cleanup(() => opener_window.close());

    // 2. Define the openee's URL as being served by the service worker.
    const openee_url = https_origin + "/common/dispatcher/proxied?" + token();

    // 3. Register, install and activate a ServiceWorker serving the openee_url.
    const sw_token = token();
    const sw_url = https_origin + executor_service_worker_path + sw_headers +
      `&uuid=${sw_token}`;
    const sw_scope = openee_url; // One-time scope, because of the token.

    const sw_registration =
      await service_worker_unregister_and_register(test, sw_url, sw_scope);
    test.add_cleanup(() => sw_registration.unregister());
    await wait_for_state(test, sw_registration.installing, 'activated');

    send(sw_token, `
      fetchHandler = event => {
        if (!event.request.url.includes("proxied"))
          return;

        const response = new Response(\`
          <script src="/common/dispatcher/dispatcher.js"></scr\`+\`ipt>
          <script>
            send("${driver_token}", opener ? "opener is set"
                                           : "opener is null");
          </scr\` + \`ipt>
        \`, {
          status: 200,
          headers: ${JSON.stringify(openee_headers)},
        });
        event.respondWith(response);
      }

      await clients.claim();

      send("${driver_token}", serviceWorker.state);
    `)
    assert_equals(await receive(driver_token), "activated");

    // 4. The opener opens a popup. Its document is a synthetic response served
    // from the Service Worker.
    send(opener_token, `
        window.open("${openee_url}");
    `);

    assert_equals(await receive(driver_token),
     (expected_browing_context_group == swap_browsing_context_group)
            ? "opener is null"
            : "opener is set");
  }, description);
};

popupCoopBySwTest("opener:basic, openee:basic, sw:basic",
                   opener_basic, openee_basic, sw_basic,
                   keep_browsing_context_group);
popupCoopBySwTest("opener:basic, openee:basic, sw:coi",
                   opener_basic, openee_basic, sw_coi,
                   keep_browsing_context_group);
popupCoopBySwTest("opener:basic, openee:coi, sw:basic",
                   opener_basic, openee_coi, sw_basic,
                   swap_browsing_context_group);
popupCoopBySwTest("opener:basic, openee:coi, sw:coi",
                   opener_basic, openee_coi, sw_coi,
                   swap_browsing_context_group);
popupCoopBySwTest("opener:coi, openee:basic, sw:basic",
                   opener_coi, openee_basic, sw_basic,
                   swap_browsing_context_group);
popupCoopBySwTest("opener:coi, openee:basic, sw:coi",
                   opener_coi, openee_basic, sw_coi,
                   swap_browsing_context_group);
popupCoopBySwTest("opener:coi, openee:coi, sw:basic",
                   opener_coi, openee_coi, sw_basic,
                   keep_browsing_context_group);
popupCoopBySwTest("opener:coi, openee:coi, sw:coi",
                   opener_coi, openee_coi, sw_coi,
                   keep_browsing_context_group);
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
        "byte_end": 36,
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
  "source_name": "html/cross-origin-opener-policy/popup-coop-by-sw.https.html"
}
```
