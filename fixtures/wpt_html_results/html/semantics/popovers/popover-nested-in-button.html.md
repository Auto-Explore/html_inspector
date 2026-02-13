# html/semantics/popovers/popover-nested-in-button.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/popovers/popover-nested-in-button.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8" />
<link rel="author" href="mailto:masonf@chromium.org">
<link rel=help href="https://github.com/whatwg/html/pull/10770">
<link rel=help href="https://issues.chromium.org/issues/379241451">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="resources/popover-utils.js"></script>

<button id=case1 popovertarget=popover>Button
  <div popover id=popover>
    <span class=descendant>Popover</span>
  </div>
</button>

<button id=case2 popovertarget=case2 popover>Self link</button>

<button popovertarget=case3>Button
  <div popover id=case3>
    <span class=descendant>Popover</span>
  </div>
</button>

<div id=case4>
  <template shadowrootmode=open>
    <button popovertarget=case4>Button
      <div popover id=case4>
        <span class=descendant>Popover</span>
      </div>
    </button>
  </template>
</div>

<script>
promise_test(async t => {
  const invoker = document.querySelector('#case1');
  const popover = invoker.querySelector('[popover]');
  const descendant = invoker.querySelector('.descendant');
  assert_false(popover.matches(':popover-open'));
  invoker.click();
  assert_true(popover.matches(':popover-open'));
  popover.click();
  assert_true(popover.matches(':popover-open'),'Should still be open');
  descendant.click();
  assert_true(popover.matches(':popover-open'),'Should still be open, even for descendant');
  popover.hidePopover();
},'clicking a popover nested inside a button should not re-invoke the popover');

promise_test(async t => {
  const element = document.querySelector('#case2');
  assert_false(element.matches(':popover-open'));
  element.showPopover();
  assert_true(element.matches(':popover-open'));
  element.click(); // This is a click on the button, which is also the popover
  assert_false(element.matches(':popover-open'));
  element.hidePopover();
},'corner case: invoker that is also a popover');

promise_test(async t => {
  const popover = document.querySelector('#case3');
  const outerInvoker = popover.parentElement;
  const descendant = popover.querySelector('.descendant');
  const innerInvoker = popover.appendChild(document.createElement('button'));
  innerInvoker.popoverTargetElement = popover;
  assert_false(popover.matches(':popover-open'));
  outerInvoker.click();
  assert_true(popover.matches(':popover-open'));
  descendant.click();
  assert_true(popover.matches(':popover-open'),'descendant doesn\'t close popover');
  innerInvoker.click();
  assert_false(popover.matches(':popover-open'),'inner invoker still works');
},'invoker inside popover still works, even with weird nesting');

promise_test(async t => {
  const popover = document.querySelector('#case4').shadowRoot.querySelector('[popover]');
  const invoker = popover.parentElement;
  const descendant = popover.querySelector('.descendant');
  assert_false(popover.matches(':popover-open'));
  invoker.click();
  assert_true(popover.matches(':popover-open'));
  descendant.click();
  assert_true(popover.matches(':popover-open'),'descendant doesn\'t close popover');
},'invoker inside popover still works, even inside of a shadowroot');
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.child.disallowed_in_phrasing_parent",
      "message": "Element “div” not allowed as child of “button” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 458,
        "byte_start": 434,
        "col": 3,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.child.disallowed_in_phrasing_parent",
      "message": "Element “div” not allowed as child of “button” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 645,
        "byte_start": 623,
        "col": 3,
        "line": 19
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
  "source_name": "html/semantics/popovers/popover-nested-in-button.html"
}
```
