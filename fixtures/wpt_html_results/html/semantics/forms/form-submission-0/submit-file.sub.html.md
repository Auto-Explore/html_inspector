# html/semantics/forms/form-submission-0/submit-file.sub.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/form-submission-0/submit-file.sub.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body>
<iframe id=testframe name=testframe></iframe>
<form id=testform method=post action="//{{domains[www1]}}:{{location[port]}}/html/semantics/forms/form-submission-0/resources/file-submission.py" target=testframe enctype="multipart/form-data">
<input name=testinput id=testinput type=file>
</form>
<script>
async_test(t => {
  const dataTransfer = new DataTransfer();
  dataTransfer.items.add(new File(["foobar"], "name"));
  assert_equals(1, dataTransfer.files.length);

  testinput.files = dataTransfer.files;
  testform.submit();

  onmessage = t.step_func(e => {
    if (e.source !== testframe) return;
    assert_equals(e.data, "foobar");
    t.done();
  });
}, 'Posting a File');
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
  "source_name": "html/semantics/forms/form-submission-0/submit-file.sub.html"
}
```
