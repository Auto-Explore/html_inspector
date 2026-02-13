# html/browsers/history/the-location-interface/non-automated/reload_in_resize-1.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/history/the-location-interface/non-automated/reload_in_resize-1.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<p>Resize this window. FAIL if the window doesn't close shortly afterwards.</p>
<script>
onload = opener.t.step_func(function() {
  opener.load_count++;
  if (opener.load_count > 1) {
    opener.do_test();
  }
})

onresize = opener.t.step_func(function() {
  opener.flag_resized();
  location.reload();
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
  "source_name": "html/browsers/history/the-location-interface/non-automated/reload_in_resize-1.html"
}
```
