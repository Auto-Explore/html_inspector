# html/semantics/forms/the-form-element/form-indexed-element.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-form-element/form-indexed-element.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title>form.elements: indexed</title>
<link rel="author" title="Ivan.Yang" href="mailto:jsyangwenjie@gmail.com">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<div id="test">
<form id=form>
<input type="radio" name="radio1" id="r1" value=1>
<input type="radio" name="radio2" id="r2" value=2>
</form>
</div>
<script>
test(function() {
  var form = document.getElementById("form");
  assert_equals(form[0], document.getElementById("r1"));
  assert_equals(form[1], document.getElementById("r2"));
  assert_equals(form[2], undefined);
  assert_equals(form[-1], undefined);
}, "form.elements should be accessed correctly by index")

test(function(){
  var form = document.getElementById("form");
  var old_item = form[0];
  var old_desc = Object.getOwnPropertyDescriptor(form, 0);
  assert_equals(old_desc.value, old_item);
  assert_true(old_desc.enumerable);
  assert_true(old_desc.configurable);
  assert_false(old_desc.writable);

  Object.prototype[0] = 5;
  this.add_cleanup(function () { delete Object.prototype[0]; });
  assert_equals(form[0], old_item);

  delete form[0];
  assert_equals(form[0], old_item);

  assert_throws_js(TypeError, function() {
    "use strict";
    delete form[0];
  });
  assert_equals(form[0], old_item);
}, 'Trying to delete an indexed property name should never work');
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
  "source_name": "html/semantics/forms/the-form-element/form-indexed-element.html"
}
```
