# html/browsers/browsing-the-web/history-traversal/pageswap/pageswap-replace-with-cross-origin-redirect.sub.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/history-traversal/pageswap/pageswap-replace-with-cross-origin-redirect.sub.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>pageswap navigationactivation for replace navigations with a  same-origin final url with cross-origin redirects</title>
<link rel="help" href="https://html.spec.whatwg.org/">
<link rel="author" href="mailto:khushalsagar@chromium.org">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/common/get-host-info.sub.js"></script>
<script>
const params = new URLSearchParams(location.search);
// The page the popup navigates to.
const is_new_page = params.has('new');
// The initial page in the popup.
const is_popup_page = params.has('popup');
// The test page itself.
const is_test_page = !is_popup_page && !is_new_page;

const channel = new BroadcastChannel("testchannel");

if (is_test_page) {
  const expectedUrl = location.href + "?new";
  const expectedEvents = ["pageswap", "pagehide", "pagereveal", "activation"];

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
  }, `pageswap on navigation with same-origin redirect`);
} else if (is_popup_page) {
  onload = () => {
    requestAnimationFrame(() => requestAnimationFrame(() => {
      let newUrl = get_host_info().HTTPS_REMOTE_ORIGIN + "/common/redirect.py?location=" + location.href.split('?')[0] + "?new";
      location.replace(newUrl);
    }));

    onpageswap = (e) => {
      window.opener.events.push("pageswap");
      if (e.viewTransition != null)
        window.opener.events.push("transition");
      if (e.activation != null)
        window.opener.events.push("activation");
    };

    onpagehide = () => {
      window.opener.events.push("pagehide");
    };
  };
} else {
  assert_true(is_new_page);
  onpageshow = () => {
    window.opener.events.push("pagereveal");
    if (navigation.activation.from != null)
      window.opener.events.push("activation");
    channel.postMessage("nav");
  }
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
  "source_name": "html/browsers/browsing-the-web/history-traversal/pageswap/pageswap-replace-with-cross-origin-redirect.sub.html"
}
```
