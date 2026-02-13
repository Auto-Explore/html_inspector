# html/browsers/the-window-object/garbage-collection-and-browsing-contexts/non-automated/discard_iframe_history_1-1.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/the-window-object/garbage-collection-and-browsing-contexts/non-automated/discard_iframe_history_1-1.html",
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
var iframe = document.getElementsByTagName("iframe")[0];
var history_length;

function load_frame(src) {
   history_length = history.length;
   iframe.src = src;
   var button = document.getElementsByTagName("button")[0];
   button.parentNode.removeChild(button);
}

remove_frame = t.step_func(function() {
  try {
    opener.assert_equals(history.length, history_length + 1, "History length after loading page in iframe");
    iframe.parentNode.removeChild(iframe);
    opener.assert_equals(history.length, history_length, "History length after removing iframe");
    t.done();
  } finally {
    window.close();
  }
});

</script>
<button onclick="load_frame('discard_iframe_history_1-2.html')">Click here</button>
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
  "source_name": "html/browsers/the-window-object/garbage-collection-and-browsing-contexts/non-automated/discard_iframe_history_1-1.html"
}
```
