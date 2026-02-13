# html/semantics/forms/form-submission-0/url-encoded.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/form-submission-0/url-encoded.html",
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
<iframe id=testframe src="/common/blank.html"></iframe>
<script>
var simple_tests = [
  {
    name: "text.simple",
    input: "<input name=foo value=bara>",
    output: "foo=bara"
  },
  {
    name: "textarea.simple",
    input: "<textarea name=foo>bar</textarea>",
    output: "foo=bar"
  },
  {
    name: "nokeygen.simple",
    input: "<input name=foo value=barb><keygen>",
    output: "foo=barb"
  }
];
simple_tests.forEach(function(test_obj) {
  test_obj.test = async_test(test_obj.name);
});
function run_simple_test() {
  if (simple_tests.length == 0) {
    return;
  }
  test_obj = simple_tests.pop();
  var t = test_obj.test;
  var testframe = document.getElementById("testframe");
  var testdocument = testframe.contentWindow.document;
  testdocument.body.innerHTML =
    "<form id=testform action=\"/common/blank.html\">" +
    test_obj.input +
    "</form>";
  testframe.onload = function() {
    t.step(function (){
      var get_url = testframe.contentWindow.location.toString();
      var encoded = get_url.substr(get_url.indexOf("?") + 1);
      assert_equals(encoded, test_obj.output);
    });
    t.done();
    run_simple_test();
  };
  testdocument.getElementById("testform").submit();
}
document.getElementById("testframe").onload = run_simple_test;
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
  "source_name": "html/semantics/forms/form-submission-0/url-encoded.html"
}
```
