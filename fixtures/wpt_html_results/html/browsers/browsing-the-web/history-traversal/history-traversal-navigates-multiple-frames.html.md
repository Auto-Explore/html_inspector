# html/browsers/browsing-the-web/history-traversal/history-traversal-navigates-multiple-frames.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/history-traversal/history-traversal-navigates-multiple-frames.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<iframe id="i" src="/common/blank.html"></iframe>
<script>
async_test(t => {
  window.onload = () => t.step_timeout(t.step_func(() => {
    let starting_history_length = history.length;
    location.hash = "#a";
    assert_equals(starting_history_length + 1, history.length);
    i.contentWindow.location.hash = "#b";
    assert_equals(starting_history_length + 2, history.length);

    let popstateCount = 0;
    const popstateCalled = t.step_func(() => {
      popstateCount++;
      if (popstateCount < 2)
        return;
      assert_equals(location.hash, "");
      assert_equals(i.contentWindow.location.hash, "");
      t.done();
    });

    window.onpopstate = popstateCalled;
    i.contentWindow.onpopstate = popstateCalled;
    history.go(-2);
  }), 0);
}, "A history traversal should be able to navigate a parent and child simultaneously");
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
  "source_name": "html/browsers/browsing-the-web/history-traversal/history-traversal-navigates-multiple-frames.html"
}
```
