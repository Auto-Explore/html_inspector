# html/browsers/the-window-object/garbage-collection-and-browsing-contexts/discard_iframe_history_1-1.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/the-window-object/garbage-collection-and-browsing-contexts/discard_iframe_history_1-1.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<iframe></iframe>
<script>
var t = opener.t;

onload = t.step_func(function() {
  setTimeout(t.step_func(function() {
     var history_length = history.length;
     var iframe = document.getElementsByTagName("iframe")[0];
     iframe.onload = t.step_func(function() {
       opener.assert_equals(history.length, history_length, "History length before iframe removal");
       iframe.parentNode.removeChild(iframe);
       opener.assert_equals(history.length, history_length, "History length after iframe removal");
       t.done();
       window.close();
     });
     iframe.src = "discard_iframe_history_1-2.html;";
  }), 100);
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
  "source_name": "html/browsers/the-window-object/garbage-collection-and-browsing-contexts/discard_iframe_history_1-1.html"
}
```
