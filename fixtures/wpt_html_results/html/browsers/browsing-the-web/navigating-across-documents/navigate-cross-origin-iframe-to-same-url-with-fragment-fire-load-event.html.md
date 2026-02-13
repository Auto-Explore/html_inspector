# html/browsers/browsing-the-web/navigating-across-documents/navigate-cross-origin-iframe-to-same-url-with-fragment-fire-load-event.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/navigating-across-documents/navigate-cross-origin-iframe-to-same-url-with-fragment-fire-load-event.html",
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
<body>
<script>
async_test(t => {
  const crossOriginUrl = new URL(get_host_info().HTTPS_REMOTE_ORIGIN);
  crossOriginUrl.pathname = "/common/blank.html";
  const i = document.createElement("iframe");
  i.src = crossOriginUrl;
  document.body.appendChild(i);

  let wasLoadEventFired = false;
  i.onload = t.step_func(() => {
    // Though iframe is cross-origin and changing hash leads soft reload, the
    // load event should be fired to protect sensitive information.
    // See: https://crbug.com/1248444
    crossOriginUrl.hash = "#foo";
    i.onload = () => {
      assert_false(wasLoadEventFired)
      wasLoadEventFired = true;
      // Wait for a while to ensure other onload events are never fired.
      t.step_timeout(() => t.done(), 100);
    };
    i.src = crossOriginUrl;
  });

}, "Changing the URL hash of a cross-origin iframe should fire a load event");
</script>
</body>
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
  "source_name": "html/browsers/browsing-the-web/navigating-across-documents/navigate-cross-origin-iframe-to-same-url-with-fragment-fire-load-event.html"
}
```
