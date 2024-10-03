import { forwardRef, useMemo } from "react";
import { useNavigate } from "react-router-dom";
import {TokenHelper} from '@/plugins/helper/token-helper';

interface Props {
  children?: React.ReactNode | React.ReactNodeArray;
  alt?: React.ReactNode | React.ReactNodeArray;
  groups?: string[];
  perms?: string[];
  rule?: (user: any) => boolean; // Optional custom rule function
  redirectToLogin?: boolean; // Option to redirect unauthenticated users
}

const ProtectedContent = forwardRef<HTMLDivElement, Props>((props, ref) => {
  const { children, groups, perms, alt, rule, redirectToLogin = true } = props;
  const { authenticated, claims } = TokenHelper.parseUserToken(TokenHelper.getAccessToken());
  const navigate = useNavigate();

  // Redirect unauthenticated users to the login page if specified
  if (!authenticated) {
    if (redirectToLogin) {
      navigate("/login");
      return null; // Prevent further rendering
    }
  }

  // groups the user is a member of
  const userGroups = useMemo(() => {
    return user?.groups?.map((g: any) => g.name) || [];
  }, [user]);

  // all user permissions
  const permsSet = useMemo(() => {
    return user?.user_permissions || [];
  }, [user]);

  let showContent = false;

  // Grant access to superusers/admins
  if (user?.is_superuser) {
    showContent = true;
  } else {
    // Check group membership
    if (groups) {
      showContent = userGroups.some((group: string) => groups.includes(group));
    }

    // Check user permissions
    if (perms && !showContent) {
      showContent = permsSet.some((perm: string) => perms.includes(perm));
    }

    // Check custom rule if provided
    if (rule && !showContent) {
      showContent = rule(user);
    }
  }

  // Render the protected content or fallback
  return showContent ? <>{children}</> : <>{alt}</>;
});

export default ProtectedContent;
