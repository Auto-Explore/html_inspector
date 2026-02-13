# html/semantics/forms/attributes-common-to-form-controls/dirname-rtl-manual.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/attributes-common-to-form-controls/dirname-rtl-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>Submitting element directionality: the dirname attribute (rtl)</title>
<link rel="author" title="Denis Ah-Kang" href="mailto:denis@w3.org">
<link rel=help href="https://html.spec.whatwg.org/multipage/#submitting-element-directionality:-the-dirname-attribute">
<form action="dirname-rtl-manual.html" method=get>
  <p><label>User: <input type=text name="user" dirname="user.dir" required></label></p>
  <p><label>Comment: <textarea name="comment" dirname="comment.dir" required></textarea></label></p>
  <p><button type=submit>Post Comment</button></p>
</form>
<p>Switch to a right-to-left writing direction, enter a text in the input and textarea, and submit the form.</p>
<p>Test passes if the word "PASS" appears below</p>
<script>
  function getParameterByName(name) {
    name = name.replace(/[\[]/, "\\[").replace(/[\]]/, "\\]");
    var regex = new RegExp("[\\?&]" + name + "=([^&#]*)"),
    results = regex.exec(location.search);
    return results == null ? "" : decodeURIComponent(results[1].replace(/\+/g, " "));
  }

  var userDir = getParameterByName("user.dir");
  var commentDir = getParameterByName("comment.dir");
  if (commentDir && userDir) {
    var p = document.createElement("p");
    var success = (commentDir == "rtl" && userDir == "rtl")
    p.textContent = success ? "PASS" : "FAIL";
    document.body.appendChild(p);
  }
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
  "source_name": "html/semantics/forms/attributes-common-to-form-controls/dirname-rtl-manual.html"
}
```
