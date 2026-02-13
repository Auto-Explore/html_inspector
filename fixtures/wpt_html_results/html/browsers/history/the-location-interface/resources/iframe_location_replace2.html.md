# html/browsers/history/the-location-interface/resources/iframe_location_replace2.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/history/the-location-interface/resources/iframe_location_replace2.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>replace2.html
<script>
  let pageshowCount = 0;
  onpageshow = parent.t.step_func(() => {
      pageshowCount += 1;
      if (pageshowCount == 2) {
          parent.t.done();
      }
  });
  onload = parent.t.step_func(() => {
      history.back();
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
  "source_name": "html/browsers/history/the-location-interface/resources/iframe_location_replace2.html"
}
```
