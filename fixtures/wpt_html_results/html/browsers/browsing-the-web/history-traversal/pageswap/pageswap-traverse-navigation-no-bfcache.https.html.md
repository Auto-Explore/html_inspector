# html/browsers/browsing-the-web/history-traversal/pageswap/pageswap-traverse-navigation-no-bfcache.https.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/history-traversal/pageswap/pageswap-traverse-navigation-no-bfcache.https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>pageswap navigationactivation for traverse navigations</title>
<link rel="help" href="https://html.spec.whatwg.org/">
<link rel="author" href="mailto:khushalsagar@chromium.org">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/common.js"></script>
<script src="/html/browsers/browsing-the-web/back-forward-cache/resources/disable_bfcache.js"></script>
<style></style>
<script>
const channel = new BroadcastChannel("testchannel");

const params = new URLSearchParams(location.search);
const is_initial_page_first_navigation = params.has('popup') && navigation.entries().length == 1;
const is_new_page = params.has('new');
const is_test_page = !params.has('popup') && !params.has('new');

// The test page which opens a popup for the navigation sequence.
if (is_test_page) {
  const expectedUrl = location.href.split('?')[0] + "?popup";
  const expectedEvents = ["pageswap", expectedUrl, "traverse","from", "pagehide"];

  promise_test(async t => {
    let popup;
    onload = () => {
      window.events = [];
      popup = window.open("?popup");
    };

    await new Promise(resolve => {
      channel.addEventListener(
        "message", t.step_func(async (e) => {
          if (e.data === "nav") {
            assert_array_equals(window.events, expectedEvents, 'incorrect event order');
            popup.close();
            resolve();
          }
      }));
    });
  }, `pageswap on traverse navigation from script`);
} else if (is_initial_page_first_navigation) {
  // The popup page which the user navigates back to.
  onload = async () => {
    await disableBFCache();
    requestAnimationFrame(() => requestAnimationFrame(() => {
      location.href = location.href.split('?')[0] + '?new';
    }));
  };

  onpageshow = (e) => {
    assert_false(e.persisted, 'the test should run without BFCache');
  }
} else if (is_new_page) {
  onload = () => {
    requestAnimationFrame(() => requestAnimationFrame(() => {
      navigation.back();
    }));
  };

  onpageswap = (e) => {
    window.opener.events.push("pageswap");
    if (e.viewTransition != null)
      window.opener.events.push("transition");
    window.opener.events.push(e.activation.entry.url);
    window.opener.events.push(e.activation.navigationType);
    if (e.activation.from == navigation.currentEntry)
      window.opener.events.push("from");
  };

  onpagehide = () => {
    window.opener.events.push("pagehide");
    channel.postMessage("nav");
  };
}
</script>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/browsers/browsing-the-web/history-traversal/pageswap/pageswap-traverse-navigation-no-bfcache.https.html"
}
```
