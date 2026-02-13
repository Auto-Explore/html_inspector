# html/dom/aria-element-reflection-disconnected.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/aria-element-reflection-disconnected.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<meta charset="utf-8" />
<link rel=help href="https://html.spec.whatwg.org/#attr-associated-element">
<link rel="author" href="masonf@chromium.org">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<div id=single_element>
  <input aria-activedescendant=foo>
  <p id=foo></p>
</div>

<div id=array>
  <input aria-describedby="foo1 foo2">
  <div id=targets>
    <p id=foo1></p>
    <p id=foo2></p>
  </div>
</div>

<script>
  function isIterable(obj) {
    if (obj === null)
      return false;
    return typeof obj[Symbol.iterator] === 'function';
  }
  function toSet(elementOrList) {
    if (!isIterable(elementOrList)) {
      return new Set([elementOrList]);
    }
    return new Set(elementOrList);
  }
  function assert_equal_elements(arr1, arr2, msg) {
    msg = msg || "Arrays not equal";
    arr1 = toSet(arr1);
    arr2 = toSet(arr2);
    assert_true(arr1.size === arr2.size &&
        [...arr1].every((x) => arr2.has(x)), msg);
  }
  function single_test(container, targets, contentAttr, idlAttr, isSingleTarget) {
    // Start with idref-based reference:
    const el = container.querySelector('input');
    assert_true(el.getAttribute(contentAttr) != null && el.getAttribute(contentAttr) != '','Should start with idref attribute');
    assert_equal_elements(el[idlAttr],targets);
    container.remove();
    assert_equal_elements(el[idlAttr],targets,'idrefs should continue to work when target is disconnected');
    document.body.appendChild(container);
    assert_equal_elements(el[idlAttr],targets,'functional when reconnected');

    // Now set up an attr-associated element:
    el[idlAttr] = isSingleTarget ? targets[0] : targets;
    assert_equal_elements(el[idlAttr],targets);
    assert_equals(el.getAttribute(contentAttr),'','Content attribute is present but empty');
    container.remove();
    assert_equal_elements(el[idlAttr],targets,'attr-associated element still functional');
    assert_equals(el.getAttribute(contentAttr),'','Attribute still blank');
    document.body.appendChild(container);
    assert_equal_elements(el[idlAttr],targets,'still functional when reconnected');

    // Sanity check:
    el.removeAttribute(contentAttr);
    assert_equal_elements(el[idlAttr],null);
    assert_equals(el.getAttribute(contentAttr),null);
    container.remove();
    assert_equal_elements(el[idlAttr],null);
    assert_equals(el.getAttribute(contentAttr),null);
    document.body.appendChild(container);
  }

  test(() => {
    const container = document.getElementById('single_element');
    const targets = [container.querySelector('#foo')];
    single_test(container, targets, 'aria-activedescendant', 'ariaActiveDescendantElement', true);
  },'Element references should stay valid when content is disconnected (single element)');

  test(() => {
    const container = document.getElementById('array');
    const targets = Array.from(container.querySelector('#targets').children);
    single_test(container, targets, 'aria-describedby', 'ariaDescribedByElements', false);
  },'Element references should stay valid when content is disconnected (element array)');
</script>

```

```json
{
  "messages": [
    {
      "category": "Aria",
      "code": "aria.activedescendant.descendant_or_owns",
      "message": "Attribute “aria-activedescendant” value should either refer to a descendant element, or should be accompanied by attribute “aria-owns”.",
      "severity": "Warning",
      "span": {
        "byte_end": 331,
        "byte_start": 298,
        "col": 3,
        "line": 9
      }
    },
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
  "source_name": "html/dom/aria-element-reflection-disconnected.html"
}
```
