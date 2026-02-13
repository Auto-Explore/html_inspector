# html/editing/editing-0/writing-suggestions/writingsuggestions.html

Counts:
- errors: 2
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/editing-0/writing-suggestions/writingsuggestions.html",
  "validated_html_truncated": true,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Tests for the writingsuggestions attribute</title>
<link rel='author' title='Sanket Joshi' href='mailto:sajos@microsoft.com'>
<link rel="help" href="https://html.spec.whatwg.org/multipage/interaction.html#writingsuggestions">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
'use strict';

customElements.define('test-custom-element', class extends HTMLElement {});

test(function() {
    assert_true('writingSuggestions' in document.createElement('input'));
}, 'Test that the writingsuggestions attribute is available on HTMLInputElement.');

test(function() {
    assert_true('writingSuggestions' in document.createElement('textarea'));
}, 'Test that the writingsuggestions attribute is available on HTMLTextAreaElement.');

test(function() {
    assert_true('writingSuggestions' in document.createElement('div'));
}, 'Test that the writingsuggestions attribute is available on HTMLDivElement.');

test(function() {
    assert_true('writingSuggestions' in document.createElement('span'));
}, 'Test that the writingsuggestions attribute is available on HTMLSpanElement.');

test(function() {
    assert_true('writingSuggestions' in document.createElement('test-custom-element'));
}, 'Test that the writingsuggestions attribute is available on custom elements.');

test(function() {
    let input = document.createElement('input');
    input.type = 'color';
    assert_true('writingSuggestions' in input);
}, 'Test that the writingsuggestions attribute is available on an input type which the attribute doesn\'t apply. The User Agent is responsible that writing suggestions are not applied to the element.');

test(function() {
    let textarea = document.createElement('textarea');
    textarea.disabled = true;
    assert_true('writingSuggestions' in textarea);
}, 'Test that the writingsuggestions attribute is available on a disabled element. The User Agent is responsible that writing suggestions are not applied to the element.');

function testSetAttributeDirectly(IDLValue, contentValue, expectedIDLValue, expectedContentValue, testDescription) {
  test(function() {
      let input_color = document.createElement('input');
      input_color.type = 'color';

      let disabled_textarea = document.createElement('textarea');
      disabled_textarea.disabled = true;

      const elements = [document.createElement('input'),
                        document.createElement('textarea'),
                        document.createElement('div'),
                        document.createElement('span'),
                        document.createElement('test-custom-element'),
                        disabled_textarea,
                        input_color ];

      elements.forEach(function(element) {
        if (IDLValue != undefined) {
          element.writingSuggestions = IDLValue;
        }
        if (contentValue != undefined) {
          element.setAttribute('writingsuggestions', contentValue);
        }
        assert_equals(element.writingSuggestions, expectedIDLValue);
        assert_equals(element.getAttribute('writingsuggestions'), expectedContentValue);
      });
  }, testDescription);
}

// Test setting either the `writingsuggestions` IDL or content attribute to some variation of 'true' directly on the target element.
testSetAttributeDirectly('true', undefined, 'true', 'true', 'Test setting the `writingsuggestions` IDL attribute to `true` directly on the target element.');
testSetAttributeDirectly(undefined, 'true', 'true', 'true', 'Test setting the `writingsuggestions` content attribute to `true` directly on the target element.');
testSetAttributeDirectly(true, undefined, 'true', 'true', 'Test setting the `writingsuggestions` IDL attribute to boolean `true` directly on the target element.');
testSetAttributeDirectly(undefined, true, 'true', 'true', 'Test setting the `writingsuggestions` content attribute to boolean `true` directly on the target element.');
testSetAttributeDirectly('TrUe', undefined, 'true', 'TrUe', 'Test setting the `writingsuggestions` IDL attribute to `TrUe` directly on the target element.');
testSetAttributeDirectly(undefined, 'TrUe', 'true', 'TrUe', 'Test setting the `writingsuggestions` content attribute to `TrUe` directly on the target element.');

// Test setting either the `writingsuggestions` IDL or content attribute to some variation of 'false' directly on the target element.
testSetAttributeDirectly('false', undefined, 'false', 'false', 'Test setting the `writingsuggestions` IDL attribute to `false` directly on the target element.');
testSetAttributeDirectly(undefined, 'false', 'false', 'false', 'Test setting the `writingsuggestions` content attribute to `false` directly on the target element.');
testSetAttributeDirectly(false, undefined, 'false', 'false', 'Test setting the `writingsuggestions` IDL attribute to boolean `false` directly on the target element.');
testSetAttributeDirectly(undefined, false, 'false', 'false', 'Test setting the `writingsuggestions` content attribute to boolean `false` directly on the target element.');
testSetAttributeDirectly('FaLsE', undefined, 'false', 'FaLsE', 'Test setting the `writingsuggestions` IDL attribute to `FaLsE` directly on the target element.');
testSetAttributeDirectly(undefined, 'FaLsE', 'false', 'FaLsE', 'Test setting the `writingsuggestions` content attribute to `FaLsE` directly on the target element.');

// Test setting either the `writingsuggestions` IDL or content attribute to the empty string directly on the target element.
testSetAttributeDirectly('', undefined, 'true', '', 'Test setting the `writingsuggestions` IDL attribute to the empty string directly on the target element.');
testSetAttributeDirectly(undefined, '', 'true', '', 'Test setting the `writingsuggestions` content attribute to the empty string directly on the target element.');

// Test setting either the `writingsuggestions` IDL or content attribute to an invalid value directly on the target element.
testSetAttributeDirectly('foo', undefined, 'true', 'foo', 'Test setting the `writingsuggestions` IDL attribute to an invalid value directly on the target element.');
testSetAttributeDirectly(undefined, 'foo', 'true', 'foo', 'Test setting the `writingsuggestions` content attribute to an invalid value directly on the target element.');

// Test setting neither the `writingsuggestions` IDL nor content attribute directly on the target element.
testSetAttributeDirectly(undefined, undefined, 'true', null, 'Test the writing suggestions state when the `writingsuggestions` attribute is missing.');

// Test setting the content attribute after the IDL attribute and making sure the IDL and content attributes are properly reflected.
testSetAttributeDirectly('true', 'false', 'false', 'false', 'Test setting the `writingsuggestions` content attribute to `false` after the IDL attribute was set to `true`.');
testSetAttributeDirectly('true', '', 'true', '', 'Test setting the `writingsuggestions` content attribute to the empty string after the IDL attribute was set to `true`.');
testSetAttributeDirectly('true', 'foo', 'true', 'foo', 'Test setting the `writingsuggestions` content attribute to an invalid value after the IDL attribute was set to `true`.');
testSetAttributeDirectly('true', 'TrUe', 'true', 'TrUe', 'Test setting the `writingsuggestions` content attribute to `TrUe` after the IDL attribute was set to `true`.');
testSetAttributeDirectly('true', 'FaLsE', 'false', 'FaLsE', 'Test setting the `writingsuggestions` content attribute to `FaLsE` after the IDL attribute was set to `true`.');
testSetAttributeDirectly('true', true, 'true', 'true', 'Test setting the `writingsuggestions` content attribute to boolean `true` after the IDL attribute was set to `true`.');
testSetAttributeDirectly('true', false, 'false', 'false', 'Test setting the `writingsuggestions` content attribute to boolean `false` after the IDL attribute was set to `true`.');

testSetAttributeDirectly('false', 'true', 'true', 'true', 'Test setting the `writingsuggestions` content attribute to `true` after the IDL attribute was set to `false`.');
testSetAttributeDirectly('false', '', 'true', '', 'Test setting the `writingsuggestions` content attribute to the empty string after the IDL attribute was set to `false`.');
testSetAttributeDirectly('false', 'foo', 'true', 'foo', 'Test setting the `writingsuggestions` content attribute to an invalid value after the IDL attribute was set to `false`.');
testSetAttributeDirectly('false', 'TrUe', 'true', 'TrUe', 'Test setting the `writingsuggestions` content attribute to `TrUe` after the IDL attribute was set to `false`.');
testSetAttributeDirectly('false', 'FaLsE', 'false', 'FaLsE', 'Test setting the `writingsuggestions` content attribute to `FaLsE` after the IDL attribute was set to `false`.');
testSetAttributeDirectly('false', true, 'true', 'true', 'Test setting the `writingsuggestions` content attribute to boolean `true` after the IDL attribute was set to `false`.');
testSetAttributeDirectly('false', false, 'false', 'false', 'Test setting the `writingsuggestions` content attribute to boolean `false` after the IDL attribute was set to `false`.');

test(function() {
  const elements = [ new DOMParser().parseFromString('<input writingsuggestions />', 'text/html').body.firstElementChild,
                     new DOMParser().parseFromString('<textarea writingsuggestions></textarea>', 'text/html').body.firstElementChild,
                     new DOMParser().parseFromString('<div writingsuggestions></div>', 'text/html').body.firstElementChild,
                     new DOMParser().parseFromString('<span writingsuggestions></span>', 'text/html').body.firstElementChild,
                     new DOMParser().parseFromString('<input type="color" writingsuggestions />', 'text/html').body.firstElementChild,
                     new DOMParser().parseFromString('<textarea disabled writingsuggestions></textarea>', 'text/html').body.firstElementChild ];

  elements.forEach(function(element) {
    assert_equals(element.writingSuggestions, 'true');
    assert_equals(element.getAttribute('writingsuggestions'), '');
  });
}, 'Test setting the `writingsuggestions` attribute with a missing value directly on the target element.');

test(function() {
  const elements = [ new DOMParser().parseFromString('<html><body writingsuggestions="true"><input /></body></html>', 'text/html').body.firstElementChild,
                     new DOMParser().parseFromString('<html><body writingsuggestions="true"><textarea></textarea></body></html>', 'text/html').body.firstElementChild,
                     new DOMParser().parseFromString('<html><body writingsuggestions="true"><div></div></body></html>', 'text/html').body.firstElementChild,
                     new DOMParser().parseFromString('<html><body writingsuggestions="true"><span></span></body></html>', 'text/html').body.firstElementChild,
                     new DOMParser().parseFromString('<html><body writingsuggestions="true"><input type="color"/></body></html>', 'text/html').body.firstElementChild,
                     new DOMParser().parseFromString('<html><body writingsuggestions="true"><textarea disabled></textarea></body></html>', 'text/html').body.firstElementChild ];

  elements.forEach(function(element) {
    assert_equals(element.parentElement.writingSuggestions, 'true');
    assert_equals(element.parentElement.getAttribute('writingsuggestions'), 'true');
    assert_equals(element.writingSuggestions, 'true');
    assert_equals(element.getAttribute('writingsuggestions'), null);
  });
}, 'Test setting the `writingsuggestions` attribute to "true" on a parent element.');

test(function() {
  const elements = [ new DOMParser().parseFromString('<html><body writingsuggestions=""><input /></body></html>', 'text/html').body.firstElementChild,
                     new DOMParser().parseFromString('<html><body writingsuggestions=""><textarea></textarea></body></html>', 'text/html').body.firstElementChild,
                     new DOMParser().parseFromString('<html><body writingsuggestions=""><div></div></body></html>', 'text/html').body.firstElementChild,
                     new DOMParser().parseFromString('<html><body writingsuggestions=""><span></span></body></html>', 'text/html').body.firstElementChild,
                     new DOMParser().parseFromString('<html><body writingsuggestions=""><input type="color"/></body></html>', 'text/html').body.firstElementChild,
                     new DOMParser().parseFromString('<html><body writingsuggestions=""><textarea disabled></textarea></body></html>', 'text/html').body.firstElementChild ];

  elements.forEach(function(element) {
    assert_equals(element.parentElement.writingSuggestions, 'true');
    assert_equals(element.parentElement.getAttribute('writingsuggestions'), '');
    assert_equals(element.writingSuggestions, 'true');
    assert_equals(element.getAttribute('writingsuggestions'), null);
  });
}, 'Test setting the `writingsuggestions` attribute to an empty string on a parent element.');

test(function() {
  const elements = [ new DOMParser().parseFromString('<html><body writingsuggestions="false"><input /></body></html>', 'text/html').body.firstElementChild,
                     new DOMParser().parseFromString('<html><body writingsuggestions="false"><textarea></textarea></body></html>', 'text/html').body.firstElementChild,
                     new DOMParser().parseFromString('<html><body writingsuggestions="false"><div></div></body></html>', 'text/html').body.firstElementChild,
                     new DOMParser().parseFromString('<html><body writingsuggestions="false"><span></span></body></html>', 'text/html').body.firstElementChild,
                     new DOMParser().parseFromString('<html><body writingsuggestions="false"><input type="color"/></body></html>', 'text/html').body.firstElementChild,
                     new DOMParser().parseFromString('<html><body writingsuggestions="false"><textarea disabled></textarea></body></html>', 'text/html').body.firstElementChild ];

  elements.forEach(function(element) {
    assert_equals(element.parentElement.writingSuggestions, 'false');
    assert_equals(element.parentElement.getAttribute('writingsuggestions'), 'false');
    assert_equals(element.writingSuggestions, 'false');
    assert_equals(element.getAttribute('writingsuggestions'), null);
  });
}, 'Test setting the `writingsuggestions` attribute to "false" on a parent element.');

test(function() {
  const elements = [ new DOMParser().parseFromString('<html><body writingsuggestions="foo"><input /></body></html>', 'text/html').body.firstElementChild,
                     new DOMParser().parseFromString('<html><body writingsuggestions="foo"><textarea></textarea></body></html>', 'text/html').body.firstElementChild,
                     new DOMParser().parseFromString('<html><body writingsuggestions="foo"><div></div></body></html>', 'text/html').body.firstElementChild,
                     new DOMParser().parseFromString('<html><body writingsuggestions="foo"><span></span></body></html>', 'text/html').body.firstElementChild,
                     new DOMParser().parseFromString('<html><body writingsuggestions="foo"><input type="color"/></body></html>', 'text/html').body.firstElementChild,
                     new DOMParser().parseFromString('<html><body writingsuggestions="foo"><textarea disabled></textarea></body></html>', 'text/html').body.firstElementChild ];

  elements.forEach(function(element) {
    assert_equals(element.parentElement.writingSuggestions, 'true');
    assert_equals(element.parentElement.getAttribute('writingsuggestions'), 'foo');
    assert_equals(element.writingSuggestions, 'true');
    assert_equals(element.getAttribute('writingsuggestions'), null);
  });
}, 'Test setting the `writingsuggestions` attribute to an invalid value on a parent element.');

test(function() {
  const elements = [ new DOMParser().parseFromString('<html><body writingsuggestions="true"><input writingsuggestions="false"/></body></html>', 'text/html').body.firstElementChild,
                     new DOMParser().parseFromString('<html><body writingsuggestions="true"><textarea writingsuggestions="false"></textarea></body></html>', 'text/html').body.firstElementChild,
                     new DOMParser().parseFromString('<html><body wr
```
(validated HTML truncated to first 16384 bytes)

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “body”.",
      "severity": "Error",
      "span": {
        "byte_end": 49423,
        "byte_start": 49416,
        "col": 1,
        "line": 598
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “html”.",
      "severity": "Error",
      "span": {
        "byte_end": 49431,
        "byte_start": 49424,
        "col": 1,
        "line": 599
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
  "source_name": "html/editing/editing-0/writing-suggestions/writingsuggestions.html"
}
```
