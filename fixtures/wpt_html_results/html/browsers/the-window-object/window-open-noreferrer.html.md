# html/browsers/the-window-object/window-open-noreferrer.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/the-window-object/window-open-noreferrer.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>window.open() with "noreferrer" tests</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<script>
async_test(t => {
  const channelName = "343243423432",
        channel = new BroadcastChannel(channelName);
  window.open("support/noreferrer-target.html?" + channelName, "", "noreferrer");
  channel.onmessage = t.step_func_done(e => {
    // Send message first so if asserts throw the popup is still closed
    channel.postMessage(null);

    assert_equals(e.data.name, "");
    assert_equals(e.data.referrer, "");
    assert_equals(e.data.haveOpener, false);
  });
});
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
  "source_name": "html/browsers/the-window-object/window-open-noreferrer.html"
}
```
