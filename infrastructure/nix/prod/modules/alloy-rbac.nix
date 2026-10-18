let
  name = "alloy";
  namespace = "kube-system";
  labels = {
    "app.kubernetes.io/name" = name;
    "app.kubernetes.io/component" = "monitoring";
    "app.kubernetes.io/part-of" = "fullstack-rust-react-template";
  };
in
[
  {
    apiVersion = "v1";
    kind = "ServiceAccount";
    metadata = { inherit name namespace labels; };
  }
  {
    apiVersion = "v1";
    kind = "Secret";
    type = "kubernetes.io/service-account-token";
    metadata = {
      name = "${name}-token";
      inherit namespace labels;
      annotations."kubernetes.io/service-account.name" = name;
    };
  }
  {
    apiVersion = "rbac.authorization.k8s.io/v1";
    kind = "ClusterRole";
    metadata = { inherit name labels; };
    rules = [
      {
        apiGroups = [ "" ];
        resources = [
          "pods"
          "namespaces"
          "nodes"
          "events"
        ];
        verbs = [
          "get"
          "list"
          "watch"
        ];
      }
      {
        apiGroups = [ "" ];
        resources = [ "nodes/metrics" ];
        verbs = [ "get" ];
      }
    ];
  }
  {
    apiVersion = "rbac.authorization.k8s.io/v1";
    kind = "ClusterRoleBinding";
    metadata = { inherit name labels; };
    roleRef = {
      apiGroup = "rbac.authorization.k8s.io";
      kind = "ClusterRole";
      inherit name;
    };
    subjects = [
      {
        kind = "ServiceAccount";
        inherit name namespace;
      }
    ];
  }
]
