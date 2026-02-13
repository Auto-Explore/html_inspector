# html/semantics/forms/form-submission-target/rel-form-target.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/form-submission-target/rel-form-target.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>&lt;form rel target></title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src=resources/reltester.js></script>
<div id=log></div>
<form action=resources/endpoint.html target=_blank><input type=hidden name=channelname></form>
<script>
const submitter = document.querySelector("form"),
      channelInput = document.querySelector("input");
relTester(submitter, channelInput, "<form target>");
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
  "source_name": "html/semantics/forms/form-submission-target/rel-form-target.html"
}
```
