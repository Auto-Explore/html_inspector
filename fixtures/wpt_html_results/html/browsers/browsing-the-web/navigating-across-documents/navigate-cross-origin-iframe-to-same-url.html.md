# html/browsers/browsing-the-web/navigating-across-documents/navigate-cross-origin-iframe-to-same-url.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/navigating-across-documents/navigate-cross-origin-iframe-to-same-url.html",
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
  let starting_history_length = history.length;
  let cross_origin_url = new URL(get_host_info().HTTPS_REMOTE_ORIGIN);
  cross_origin_url.pathname = "/common/blank.html";
  let i = document.createElement("iframe");
  i.src = cross_origin_url;
  document.body.appendChild(i);

  window.onload = () => t.step_timeout(() => {
    assert_equals(starting_history_length, history.length);
    i.onload = t.step_func_done(() => assert_equals(starting_history_length + 1, history.length));
    i.src = cross_origin_url;
  }, 0);
}, "Navigating a cross-origin iframe to its current url should not replace");
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
  "source_name": "html/browsers/browsing-the-web/navigating-across-documents/navigate-cross-origin-iframe-to-same-url.html"
}
```
