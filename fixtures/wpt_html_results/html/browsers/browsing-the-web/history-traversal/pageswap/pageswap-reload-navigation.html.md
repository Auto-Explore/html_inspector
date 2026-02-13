# html/browsers/browsing-the-web/history-traversal/pageswap/pageswap-reload-navigation.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/history-traversal/pageswap/pageswap-reload-navigation.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>pageswap navigationactivation for replace navigations</title>
<link rel="help" href="https://html.spec.whatwg.org/">
<link rel="author" href="mailto:khushalsagar@chromium.org">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
const expectedUrl = location.href + '?new';

const params = new URLSearchParams(location.search);
// The initial page in the popup.
const is_popup_page = params.has('popup') && !window.opener.didreload;
// The test page itself.
const is_test_page = !params.has('popup');

const channel = new BroadcastChannel("testchannel");

if (is_test_page) {
  const expectedUrl = location.href.split('?')[0] + "?popup";
  const expectedEvents = ["pageswap", "entry", "reload","from", "pagehide"];

  promise_test(async t => {
    let popup;
    onload = () => {
      window.events = [];
      window.didreload = false;
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
  }, `pageswap on replace navigation from script`);
} else if (is_popup_page) {
  onload = () => {
    requestAnimationFrame(() => requestAnimationFrame(() => {
      window.opener.didreload = true;
      location.reload();
    }));

    onpageswap = (e) => {
      window.opener.events.push("pageswap");
      if (e.viewTransition != null)
        window.opener.events.push("transition");
      if (e.activation.entry == navigation.currentEntry)
        window.opener.events.push("entry");
      window.opener.events.push(e.activation.navigationType);
      if (e.activation.from == navigation.currentEntry)
        window.opener.events.push("from");
    };

    onpagehide = () => {
      window.opener.events.push("pagehide");
      channel.postMessage("nav");
    };
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
  "source_name": "html/browsers/browsing-the-web/history-traversal/pageswap/pageswap-reload-navigation.html"
}
```
