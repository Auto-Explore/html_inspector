# html/semantics/forms/the-input-element/email.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/email.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Input Email</title>
<link rel="author" title="Kazuki Kanamori" href="mailto:yogurito@gmail.com">
<link rel="author" title="Denis Ah-Kang" href="mailto:denis@w3.org">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#e-mail-state-(type=email)">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<input type="email" id="single_email" value="user@example.com"/>
<input type="email" id="multiple_email" value="user1@example.com, user2@test.com" multiple/>
<div id="log"></div>

<script type="text/javascript">
  var single = document.getElementById('single_email'),
      mult = document.getElementById('multiple_email');

  test(function(){
    assert_false(single.multiple);
  }, "single_email doesn't have the multiple attribute");

  test(function(){
    single.value = 'user2@example.com\u000A';
    assert_equals(single.value, 'user2@example.com');
    single.value = 'user3@example.com\u000D';
    assert_equals(single.value, 'user3@example.com');
  }, 'value should be sanitized: strip line breaks');

  test(function(){
    single.value = 'user4@example.com';
    assert_true(single.validity.valid);
    single.value = 'example.com';
    assert_false(single.validity.valid);
  }, 'Email address validity');

  test(function(){
    single.setAttribute('multiple', true);
    single.value = '  user@example.com  , user2@example.com  ';
    assert_equals(single.value, 'user@example.com,user2@example.com');
    single.removeAttribute('multiple');
    assert_equals(single.value, 'user@example.com,user2@example.com');
  }, 'When the multiple attribute is removed, the user agent must run the value sanitization algorithm');

  test(function(){
    assert_true(mult.multiple);
  }, "multiple_email has the multiple attribute");

  test(function(){
    mult.value = '  user1@example.com  , user2@test.com, user3@test.com  ';
    assert_equals(mult.value, 'user1@example.com,user2@test.com,user3@test.com');
  }, "run the value sanitization algorithm after setting a new value");

  test(function(){
    mult.value = 'user1@example.com,user2@test.com,user3@test.com';
    assert_true(mult.validity.valid);

    mult.value = 'u,ser1@example.com,user2@test.com,user3@test.com';
    assert_false(mult.validity.valid);
  }, "valid value is a set of valid email addresses separated by a single ','");

  test(function(){
    mult.removeAttribute('multiple');
    mult.value = 'user1@example.com , user2@example.com';
    assert_equals(mult.value, 'user1@example.com , user2@example.com');
    mult.setAttribute('multiple', true);
    assert_equals(mult.value, 'user1@example.com,user2@example.com');
  }, 'When the multiple attribute is set, the user agent must run the value sanitization algorithm');
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 597,
        "byte_start": 566,
        "col": 1,
        "line": 12
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/forms/the-input-element/email.html"
}
```
