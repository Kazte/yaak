import type { Environment } from '@yaakapp-internal/models';
import { useSetAtom } from 'jotai';
import { trackEvent } from '../lib/analytics';
import { invokeCmd } from '../lib/tauri';
import { useActiveEnvironment } from './useActiveEnvironment';
import { useActiveWorkspace } from './useActiveWorkspace';
import { environmentsAtom } from './useEnvironments';
import { useFastMutation } from './useFastMutation';
import { usePrompt } from './usePrompt';
import { updateModelList } from './useSyncModelStores';

export function useCreateEnvironment() {
  const [, setActiveEnvironmentId] = useActiveEnvironment();
  const prompt = usePrompt();
  const workspace = useActiveWorkspace();
  const setEnvironments = useSetAtom(environmentsAtom);

  return useFastMutation<Environment | null, unknown, Environment | null>({
    toastyError: true,
    mutationKey: ['create_environment'],
    mutationFn: async (baseEnvironment) => {
      if (baseEnvironment == null) {
        throw new Error('No base environment passed');
      }

      const name = await prompt({
        id: 'new-environment',
        title: 'New Environment',
        description: 'Create multiple environments with different sets of variables',
        label: 'Name',
        placeholder: 'My Environment',
        defaultValue: 'My Environment',
        confirmText: 'Create',
      });
      if (name == null) return null;

      return invokeCmd('cmd_create_environment', {
        name,
        variables: [],
        workspaceId: workspace?.id,
        environmentId: baseEnvironment.id,
      });
    },
    onSettled: () => trackEvent('environment', 'create'),
    onSuccess: async (environment) => {
      if (environment == null) return;

      // Optimistic update
      setEnvironments(updateModelList(environment));

      await setActiveEnvironmentId(environment.id);
    },
  });
}
