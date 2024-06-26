# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def balanceBST(self, root: TreeNode) -> TreeNode:
        def inorderTraversal(node):
            if node is None:
                return []
            else:
                return inorderTraversal(node.left) 
                    + [node.val]
                    + inorderTraversal(node.right)

        def getTree(arr):
            if len(arr) == 0:
                return None
            else:
                if len(arr) == 1:
                    return TreeNode(arr[0]);

        arr = inorderTraversal(root)
        return getTree(arr)