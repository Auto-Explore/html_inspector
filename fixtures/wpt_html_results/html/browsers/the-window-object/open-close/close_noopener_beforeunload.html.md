# html/browsers/the-window-object/open-close/close_noopener_beforeunload.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/the-window-object/open-close/close_noopener_beforeunload.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Running beforeunload handler in window.close() for noopener window</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
function waitForMessage(channel, name) {
  return new Promise(resolve => {
    function listener({ data }) {
      if (data.name === name) {
        channel.removeEventListener("message", listener);
        resolve(data);
      }
    };
    channel.addEventListener("message", listener);
  });
}

async_test(t => {
  let chan = new BroadcastChannel("close_noopener_beforeunload2");
  waitForMessage(chan, "beforeunload").then(t.step_func_done(data => {
    assert_equals(data.history, 2, "session history has multiple entries");
  }));
  window.open("self-close.html?navs=1&channel=close_noopener_beforeunload2", "", "noopener");
}, "closing noopener window with 2 entries");

async_test(t => {
  let chan = new BroadcastChannel("close_noopener_beforeunload1");
  waitForMessage(chan, "beforeunload").then(t.step_func_done(data => {
    assert_equals(data.history, 1, "session history has a single entry");
  }));
  window.open("self-close.html?navs=0&channel=close_noopener_beforeunload1", "", "noopener");
}, "closing noopener window with 1 entry");
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
  "source_name": "html/browsers/the-window-object/open-close/close_noopener_beforeunload.html"
}
```
