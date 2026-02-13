# html/document-isolation-policy/no-secure-context.tentative.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/document-isolation-policy/no-secure-context.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/common/get-host-info.sub.js"></script>
<div id=log></div>
<script>
promise_test(async t => {
  const response = await fetch(get_host_info().HTTPS_REMOTE_ORIGIN+"/html/document-isolation-policy/resources/nothing-no-corp.js", {mode: "no-cors"});
  assert_equals(response.type, "opaque");
}, "DIP requires a secure context");
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
  "source_name": "html/document-isolation-policy/no-secure-context.tentative.html"
}
```
