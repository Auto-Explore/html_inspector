# html/browsers/the-window-object/garbage-collection-and-browsing-contexts/discard_iframe_history_4-1.html

Counts:
- errors: 1
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/the-window-object/garbage-collection-and-browsing-contexts/discard_iframe_history_4-1.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<script>
history_lengths = [];

var t = opener.t;

push_length = t.step_func(function () {
  history_lengths.push(history.length)
});

do_test = t.step_func(function () {
  try {
    var start_length = history_lengths[0];
    expected = [start_length, start_length + 1, start_length];
    opener.assert_array_equals(history_lengths, expected);
    t.done();
  } finally {
    window.close();
  }
});
</script>
<iframe src="discard_iframe_history_4-2.html"></iframe>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.start_tag_without_doctype",
      "message": "Start tag seen without seeing a doctype first. Expected “<!DOCTYPE html>”.",
      "severity": "Error",
      "span": {
        "byte_end": 8,
        "byte_start": 0,
        "col": 1,
        "line": 1
      }
    },
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
  "source_name": "html/browsers/the-window-object/garbage-collection-and-browsing-contexts/discard_iframe_history_4-1.html"
}
```
