# Feature Specification: Fund Add Search Filter

**Feature Branch**: `[007-fund-add-search]`  
**Created**: 2026-01-22  
**Status**: Draft  
**Input**: User description: "中间基金列表上面的添加按钮，如果已经在列表中，则下方列表只展示对应的基金，相当于一个查询功能"

## User Scenarios & Testing *(mandatory)*

<!--
  IMPORTANT: User stories should be PRIORITIZED as user journeys ordered by importance.
  Each user story/journey must be INDEPENDENTLY TESTABLE - meaning if you implement just ONE of them,
  you should still have a viable MVP (Minimum Viable Product) that delivers value.
  
  Assign priorities (P1, P2, P3, etc.) to each story, where P1 is the most critical.
  Think of each story as a standalone slice of functionality that can be:
  - Developed independently
  - Tested independently
  - Deployed independently
  - Demonstrated to users independently
-->

### User Story 1 - Filter existing fund on add (Priority: P1)

When I click the add button for a fund that already exists in the middle list, I want the lower list to show only that fund so I can quickly confirm it without creating a duplicate.  
当我点击添加按钮且该基金已经在中间列表中时，我希望下方列表只展示该基金，以便快速确认而不产生重复。

**Why this priority**: Preventing duplicates while helping users locate the existing fund is the core value of this feature.  
**Why this priority**：避免重复并快速定位已存在基金是本功能的核心价值。

**Independent Test**: Can be fully tested by attempting to add a known existing fund and verifying the lower list filters to that single fund.  
**Independent Test**：通过尝试添加已存在基金并验证下方列表只显示该基金即可独立测试。

**Acceptance Scenarios**:

1. **Given** the middle list already contains Fund A, **When** I click add for Fund A, **Then** the lower list shows only Fund A.  
   **给定** 中间列表已包含基金A，**当** 我对基金A点击添加，**则** 下方列表仅显示基金A。
2. **Given** the lower list is showing all funds, **When** I trigger the existing-fund add action, **Then** the list updates to a single-item view of the matching fund.  
   **给定** 下方列表显示全部基金，**当** 我触发现有基金的添加动作，**则** 列表更新为只显示匹配基金的一项。

---

### User Story 2 - Add a new fund normally (Priority: P2)

When I click the add button for a fund that is not in the middle list, I want the normal add behavior with no extra filtering in the lower list.  
当我点击添加按钮且该基金不在中间列表中时，我希望保持正常添加行为，下方列表不额外过滤。

**Why this priority**: The new filtering behavior must not disrupt the primary add flow for new funds.  
**Why this priority**：新增过滤行为不能影响新增基金的主流程。

**Independent Test**: Can be tested by adding a fund not currently in the middle list and verifying the lower list remains unfiltered.  
**Independent Test**：通过添加中间列表中不存在的基金，并确认下方列表仍显示全量即可独立测试。

**Acceptance Scenarios**:

1. **Given** Fund B is not in the middle list, **When** I click add for Fund B, **Then** the lower list does not switch to a single-item view.  
   **给定** 基金B不在中间列表中，**当** 我对基金B点击添加，**则** 下方列表不会切换为单项视图。

---

### User Story 3 - Return to full list after filtering (Priority: P3)

After the lower list is filtered to a single existing fund, I want a simple way to return to the full list view.  
当下方列表被过滤为单一已存在基金后，我希望能轻松返回全量列表视图。

**Why this priority**: Users must be able to continue browsing and adding other funds without friction.  
**Why this priority**：用户需要能继续浏览并添加其他基金，避免流程卡住。

**Independent Test**: Can be tested by triggering the filter and then using the clear action to restore the full list.  
**Independent Test**：通过触发过滤后使用清除动作恢复全量列表即可独立测试。

**Acceptance Scenarios**:

1. **Given** the lower list is filtered to Fund A, **When** I clear the filter action, **Then** the lower list shows all funds again.  
   **给定** 下方列表已过滤为基金A，**当** 我执行清除过滤动作，**则** 下方列表恢复显示全部基金。

---

[Add more user stories as needed, each with an assigned priority]

### Edge Cases

- The middle list contains a fund with the same name but different identifiers; only the exact matching fund should be filtered.  
  中间列表存在名称相同但标识不同的基金时，只应过滤到完全匹配的基金。
- The lower list does not contain the matching fund due to current filters or empty data; show an empty state with a clear message.  
  下方列表因当前筛选或数据为空而找不到匹配基金时，应显示空状态并给出清晰提示。
- The user triggers the add action repeatedly on the same existing fund; the system should remain stable and not create duplicates.  
  用户反复对同一已存在基金点击添加时，系统应稳定且不产生重复。

## Requirements *(mandatory)*

<!--
  ACTION REQUIRED: The content in this section represents placeholders.
  Fill them out with the right functional requirements.
-->

### Functional Requirements

- **FR-001**: System MUST detect that the target fund already exists in the middle list using a unique fund identifier.  
  **FR-001**：系统必须通过基金唯一标识检测目标基金已存在于中间列表。
- **FR-002**: When an existing fund is detected, the lower list MUST display only the matching fund and hide all others.  
  **FR-002**：当检测到已存在基金时，下方列表必须仅显示匹配基金并隐藏其他项。
- **FR-003**: The add action MUST NOT create a duplicate entry when the fund already exists.  
  **FR-003**：当基金已存在时，添加动作不得产生重复条目。
- **FR-004**: Users MUST have a clear, single-step way to return the lower list to its full view after filtering.  
  **FR-004**：用户必须能通过一个清晰的单步操作将下方列表恢复为全量视图。
- **FR-005**: If no matching fund appears in the lower list, the system MUST show an empty state that explains no results were found.  
  **FR-005**：若下方列表无匹配基金，系统必须显示“无结果”的空状态说明。
- **FR-006**: The system MUST provide a non-blocking indication that the fund already exists when the filter is applied.  
  **FR-006**：当应用过滤时，系统必须提供“基金已存在”的非阻塞提示。

### Key Entities *(include if feature involves data)*

- **Fund**: A single fund item identified by a unique id/code and name.  
  **基金**：通过唯一标识/代码和名称来识别的单个基金条目。
- **Middle Fund List**: The user’s current list of tracked funds used to check for duplicates.  
  **中间基金列表**：用户当前跟踪的基金列表，用于判断是否重复。
- **Lower Fund List View**: The list section that can be filtered to a single fund or show all funds.  
  **下方基金列表视图**：可被过滤为单个基金或显示全部基金的列表区域。
- **Filter State**: The temporary view state indicating whether the lower list is filtered and by which fund.  
  **过滤状态**：标记下方列表是否过滤以及过滤目标基金的临时视图状态。

## Success Criteria *(mandatory)*

<!--
  ACTION REQUIRED: Define measurable success criteria.
  These must be technology-agnostic and measurable.
-->

### Measurable Outcomes

- **SC-001**: In usability testing, 95% of attempts to add an existing fund result in a single-item lower list within 1 second.  
  **SC-001**：在可用性测试中，95% 的“添加已存在基金”操作可在 1 秒内看到下方列表仅显示该基金。
- **SC-002**: Users can return to the full lower list view in one action, with a success rate of 95% in tests.  
  **SC-002**：用户可通过一次操作恢复下方列表全量视图，测试成功率达到 95%。
- **SC-003**: Duplicate fund entries caused by the add action are reduced to 0 in acceptance testing.  
  **SC-003**：在验收测试中，由添加动作导致的重复基金条目为 0。
- **SC-004**: 90% of test users report that the existing fund is easy to locate after triggering the add action.  
  **SC-004**：90% 的测试用户认为触发添加动作后容易定位已存在基金。

## Assumptions

- The system has a unique identifier for each fund and uses it to determine duplicates.  
  系统为每只基金提供唯一标识并据此判断重复。
- The lower list normally shows all available funds before any filter is applied.  
  下方列表在未过滤时默认显示全部可用基金。
