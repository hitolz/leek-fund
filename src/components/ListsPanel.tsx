import React, { useState } from "react";
import { FundList } from "../types";
import { useTauriCommands } from "../hooks/useTauriCommands";
import { ConfirmDialog } from "./ConfirmDialog";

interface ListsPanelProps {
  lists: FundList[];
  selectedListId: number | null;
  onSelectList: (id: number | null) => void;
  onListsChange: () => void;
  showToast?: (message: string, type: "success" | "error") => void;
}

export const ListsPanel: React.FC<ListsPanelProps> = ({
  lists,
  selectedListId,
  onSelectList,
  onListsChange,
  showToast,
}) => {
  const [newListName, setNewListName] = useState("");
  const [creating, setCreating] = useState(false);
  const [editingId, setEditingId] = useState<number | null>(null);
  const [editingName, setEditingName] = useState("");
  const [pendingDelete, setPendingDelete] = useState<{
    id: number;
    name: string;
    fundCount: number;
    step: 1 | 2;
  } | null>(null);
  const [deleting, setDeleting] = useState(false);
  const { createList, renameList, deleteList } = useTauriCommands();

  const handleCreateList = async () => {
    if (!newListName.trim()) {
      if (showToast) {
        showToast("列表名称不能为空", "error");
      }
      return;
    }

    setCreating(true);
    try {
      await createList(newListName.trim());
      if (showToast) {
        showToast("列表创建成功", "success");
      }
      setNewListName("");
      onListsChange();
    } catch (error) {
      if (showToast) {
        showToast(String(error), "error");
      }
    } finally {
      setCreating(false);
    }
  };

  const handleRename = async (id: number) => {
    if (!editingName.trim()) {
      if (showToast) {
        showToast("列表名称不能为空", "error");
      }
      return;
    }

    try {
      await renameList(id, editingName.trim());
      if (showToast) {
        showToast("重命名成功", "success");
      }
      setEditingId(null);
      onListsChange();
    } catch (error) {
      if (showToast) {
        showToast(String(error), "error");
      }
    }
  };

  const handleDelete = (id: number, name: string, fundCount: number) => {
    setPendingDelete({ id, name, fundCount, step: 1 });
  };

  const handleDeleteConfirm = async () => {
    if (!pendingDelete || deleting) {
      return;
    }

    if (pendingDelete.step === 1) {
      setPendingDelete({ ...pendingDelete, step: 2 });
      return;
    }

    setDeleting(true);
    try {
      await deleteList(pendingDelete.id);
      if (showToast) {
        showToast("列表已删除", "success");
      }
      if (selectedListId === pendingDelete.id) {
        onSelectList(null);
      }
      onListsChange();
      setPendingDelete(null);
    } catch (error) {
      if (showToast) {
        showToast(String(error), "error");
      }
    } finally {
      setDeleting(false);
    }
  };

  const handleDeleteCancel = () => {
    if (deleting) {
      return;
    }
    setPendingDelete(null);
  };

  return (
    <div className="lists-panel">
      <div className="list-create">
        <input
          type="text"
          value={newListName}
          onChange={(e) => setNewListName(e.target.value)}
          placeholder="新建列表"
          maxLength={64}
          className="input"
          disabled={creating}
          onKeyPress={(e) => {
            if (e.key === "Enter") {
              handleCreateList();
            }
          }}
        />
        <button
          onClick={handleCreateList}
          disabled={creating || !newListName.trim()}
          className="button primary small"
        >
          {creating ? "创建中..." : "创建"}
        </button>
      </div>

      <div className="lists-container">
        {lists.length === 0 ? (
          <div className="empty-state">
            <p>还没有列表，创建一个吧</p>
          </div>
        ) : (
          lists.map((list) => (
            <button
              key={list.id}
              className={`list-item ${
                selectedListId === list.id ? "active" : ""
              }`}
              onClick={() => onSelectList(list.id)}
            >
              {editingId === list.id ? (
                <div className="list-edit">
                  <input
                    type="text"
                    value={editingName}
                    onChange={(e) => setEditingName(e.target.value)}
                    maxLength={64}
                    className="input"
                    autoFocus
                    onKeyPress={(e) => {
                      if (e.key === "Enter") {
                        handleRename(list.id);
                      }
                    }}
                    onClick={(e) => e.stopPropagation()}
                  />
                  <button
                    onClick={(e) => {
                      e.stopPropagation();
                      handleRename(list.id);
                    }}
                    className="icon-btn"
                  >
                    ✓
                  </button>
                  <button
                    onClick={(e) => {
                      e.stopPropagation();
                      setEditingId(null);
                    }}
                    className="icon-btn"
                  >
                    ✕
                  </button>
                </div>
              ) : (
                <>
                  <div className="list-info">
                    <span className="list-name">{list.name}</span>
                    <span className="list-meta">
                      {list.fund_codes.length} 只基金
                    </span>
                  </div>
                  <div className="list-actions">
                    <button
                      onClick={(e) => {
                        e.stopPropagation();
                        setEditingId(list.id);
                        setEditingName(list.name);
                      }}
                      className="icon-btn"
                      title="重命名"
                    >
                      ✏️
                    </button>
                    <button
                      onClick={(e) => {
                        e.stopPropagation();
                        handleDelete(list.id, list.name, list.fund_codes.length);
                      }}
                      className="icon-btn"
                      title="删除"
                    >
                      🗑️
                    </button>
                  </div>
                </>
              )}
            </button>
          ))
        )}
      </div>

      <ConfirmDialog
        open={pendingDelete !== null}
        title={pendingDelete?.step === 2 ? "最后确认" : "删除列表"}
        message={
          pendingDelete
            ? pendingDelete.step === 2
              ? pendingDelete.fundCount > 0
                ? `请再次确认删除列表"${pendingDelete.name}"，${pendingDelete.fundCount}只基金将被移除且无法恢复。`
                : `请再次确认删除列表"${pendingDelete.name}"，删除后无法恢复。`
              : pendingDelete.fundCount > 0
                ? `确定删除列表"${pendingDelete.name}"吗？将移除${pendingDelete.fundCount}只基金。`
                : `确定删除列表"${pendingDelete.name}"吗？`
            : ""
        }
        confirmText={pendingDelete?.step === 2 ? "确认删除" : "继续"}
        cancelText="取消"
        tone="danger"
        confirmDisabled={deleting}
        onConfirm={handleDeleteConfirm}
        onCancel={handleDeleteCancel}
      />
    </div>
  );
};
