# html/editing/editing-0/contenteditable/contentEditable-slotted-inherit.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/editing-0/contenteditable/contentEditable-slotted-inherit.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>contentEditable inherit from light tree parent</title>
<link rel="author" title="Rune Lillesveen" href="mailto:futhark@chromium.org">
<link rel=help href="https://html.spec.whatwg.org/multipage/#contenteditable">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<p>You should see the word PASS two times below and no FAIL.</p>
<div id="host1" contenteditable><div>FAILPASS</div></div>
<div id="host2" contenteditable><div>FAILPASS</div></div>
<script>
  test(() => {
    const root = host1.attachShadow({mode:"open"});
    root.innerHTML = "<slot></slot>";
    const text = host1.firstChild.firstChild;
    const selection = window.getSelection();
    selection.collapse(text, 0);
    selection.extend(text, 4);
    host1.focus();
    document.execCommand("delete");
    host1.blur();
    assert_equals(text.data, "PASS", "Text should be PASS after FAIL is deleted");
  }, "Slotted child of contenteditable host should be editable - slot direct child of shadow root");

  test(() => {
    const root = host2.attachShadow({mode:"open"});
    root.innerHTML = "<div><slot></slot></div>";
    const text = host2.firstChild.firstChild;
    const selection = window.getSelection();
    selection.collapse(text, 0);
    selection.extend(text, 4);
    host2.focus();
    document.execCommand("delete");
    host2.blur();
    assert_equals(text.data, "PASS", "Text should be PASS after FAIL is deleted");
  }, "Slotted child of contenteditable host should be editable - slot wrapped in shadow tree ancestor");
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
  "source_name": "html/editing/editing-0/contenteditable/contentEditable-slotted-inherit.html"
}
```
