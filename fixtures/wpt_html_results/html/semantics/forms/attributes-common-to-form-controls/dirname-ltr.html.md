# html/semantics/forms/attributes-common-to-form-controls/dirname-ltr.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/attributes-common-to-form-controls/dirname-ltr.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>Submitting element directionality: the dirname attribute</title>
<link rel="author" title="Denis Ah-Kang" href="mailto:denis@w3.org">
<link rel=help href="https://html.spec.whatwg.org/multipage/#submitting-element-directionality:-the-dirname-attribute">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="resources/dirname.js"></script>
<div id="log"></div>
<form action="resources/dirname-iframe.html" method=get target="iframe">
  <p><label>User: <input type=text name="user" dirname="user.dir" required></label></p>
  <p><label>Comment: <textarea name="comment" dirname="comment.dir" required></textarea></label></p>
  <p><button type=submit>Post Comment</button></p>
</form>
<iframe name="iframe"></iframe>
<script>
  document.querySelector("input").value = "foobar";
  document.querySelector("textarea").value = "foobar";
  document.querySelector("button").click();

  var t_inp = async_test("submit input element directionality");
  onIframeLoadedDone(t_inp, function(params) {
    assert_equals(params.get("user.dir"), "ltr");
  });

  var t_ta = async_test("submit textarea element directionality");
  onIframeLoadedDone(t_ta, function(params) {
    assert_equals(params.get("comment.dir"), "ltr");
  });
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
  "source_name": "html/semantics/forms/attributes-common-to-form-controls/dirname-ltr.html"
}
```
